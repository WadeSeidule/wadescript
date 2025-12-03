mod ast;
mod codegen;
mod lexer;
mod parser;
mod typechecker;

use ast::{Program, Statement};
use codegen::CodeGen;
use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;
use lexer::Lexer;
use parser::Parser;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use typechecker::TypeChecker;

fn load_program_with_imports(file_path: &str, imported: &mut HashSet<PathBuf>) -> Result<Program, String> {
    // Add .ws extension if not present
    let file_path_with_ext = if file_path.ends_with(".ws") {
        file_path.to_string()
    } else {
        format!("{}.ws", file_path)
    };

    let abs_path = fs::canonicalize(&file_path_with_ext).map_err(|e| format!("Cannot resolve path '{}': {}", file_path_with_ext, e))?;

    // Check for circular imports
    if imported.contains(&abs_path) {
        return Err(format!("Circular import detected: {}", file_path_with_ext));
    }
    imported.insert(abs_path.clone());

    // Read and parse the file
    let source_code = fs::read_to_string(&abs_path).map_err(|e| format!("Error reading file '{}': {}", file_path_with_ext, e))?;
    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    let mut result_program = Program::new();

    // Process import statements
    for statement in &program.statements {
        if let Statement::Import { path } = statement {
            // Resolve import path relative to the current file
            let current_dir = abs_path.parent().unwrap();

            // Add .ws extension if not present
            let import_path_with_ext = if path.ends_with(".ws") {
                path.clone()
            } else {
                format!("{}.ws", path)
            };

            let import_path = current_dir.join(&import_path_with_ext);
            let import_path_str = import_path.to_str().unwrap();

            // Extract module name from path (filename without extension)
            let module_name = Path::new(path).file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(path)
                .to_string();

            // Recursively load the imported file
            let imported_program = load_program_with_imports(import_path_str, imported)?;

            // Extract function names from imported program
            let mut function_names = Vec::new();
            for stmt in &imported_program.statements {
                if let Statement::FunctionDef { name, .. } = stmt {
                    function_names.push(name.clone());
                }
            }

            // Register this module
            result_program.modules.insert(module_name, function_names);

            // Merge statements and modules
            result_program.statements.extend(imported_program.statements);
            for (mod_name, func_names) in imported_program.modules {
                result_program.modules.insert(mod_name, func_names);
            }
        } else {
            // Add non-import statements from current file
            result_program.statements.push(statement.clone());
        }
    }

    Ok(result_program)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: wadescript <input_file.ws> [--emit-llvm]");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let emit_llvm = args.len() > 2 && args[2] == "--emit-llvm";

    let mut imported = HashSet::new();
    let program = load_program_with_imports(input_file, &mut imported).unwrap_or_else(|err| {
        eprintln!("Error loading program: {}", err);
        std::process::exit(1);
    });

    let mut type_checker = TypeChecker::new();
    if let Err(e) = type_checker.check_program(&program) {
        eprintln!("Type error: {}", e);
        std::process::exit(1);
    }

    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "wadescript_module", input_file);

    if let Err(e) = codegen.compile_program(&program) {
        eprintln!("Compilation error: {}", e);
        std::process::exit(1);
    }

    let module = codegen.get_module();

    if emit_llvm {
        println!("{}", module.print_to_string().to_string());
        return;
    }

    Target::initialize_native(&InitializationConfig::default()).unwrap();

    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).unwrap();
    // Use no optimization to preserve debug information
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::None,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    let output_base = Path::new(input_file).file_stem().unwrap().to_str().unwrap();
    let obj_file = format!("{}.o", output_base);
    let exe_file = output_base;

    target_machine
        .write_to_file(module, FileType::Object, Path::new(&obj_file))
        .unwrap();

    // Get the runtime library path (matches build profile)
    let runtime_lib = if cfg!(debug_assertions) {
        "target/debug/libwadescript_runtime.a"
    } else {
        "target/release/libwadescript_runtime.a"
    };

    // Link with clang (preserve debug information with -g)
    let output = Command::new("clang")
        .args(&["-g", &obj_file, runtime_lib, "-o", exe_file])
        .output()
        .expect("Failed to link object file with clang");

    if !output.status.success() {
        eprintln!("Linking failed:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    // On macOS, create dSYM bundle for debug info (before deleting object file!)
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("dsymutil")
            .arg(exe_file)
            .output();
    }

    // Clean up object file
    fs::remove_file(&obj_file).ok();

    println!("Compiled successfully to '{}'", exe_file);
}
