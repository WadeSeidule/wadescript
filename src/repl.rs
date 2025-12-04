//! WadeScript REPL (Read-Eval-Print Loop)
//!
//! Interactive interpreter using LLVM JIT compilation.

use std::collections::HashMap;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use inkwell::context::Context;
use inkwell::module::Module;

use crate::ast::{Type, Statement, Program, Expression};
use crate::codegen::CodeGen;
use crate::jit::JitEngine;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::typechecker::TypeChecker;

/// Persistent variable in REPL
struct ReplVariable {
    ws_type: Type,
    #[allow(dead_code)]
    ptr: *mut u8,  // Pointer to allocated memory (kept for future cleanup)
    #[allow(dead_code)]
    size: usize,   // Size of allocation (kept for future cleanup)
}

/// User-defined function info for forward declarations
struct UserFunction {
    params: Vec<(String, Type)>,  // param name and type
    return_type: Type,
}

/// REPL state and execution engine
pub struct Repl {
    /// Static context for JIT (leaked to ensure 'static lifetime)
    context: &'static Context,
    /// Type checker with persistent symbol table
    type_checker: TypeChecker,
    /// Variables persisted across REPL inputs
    variables: HashMap<String, ReplVariable>,
    /// User-defined functions for forward declarations
    user_functions: HashMap<String, UserFunction>,
    /// Functions defined in REPL (reserved for future use)
    #[allow(dead_code)]
    functions: HashMap<String, (Vec<Type>, Type)>,
    /// JIT engine for compilation and execution
    jit: JitEngine<'static>,
    /// Multi-line input buffer
    input_buffer: String,
}

impl Repl {
    /// Get the size of a WadeScript type in bytes
    fn type_size(ws_type: &Type) -> usize {
        match ws_type {
            Type::Int => 8,    // i64
            Type::Float => 8,  // f64
            Type::Bool => 1,   // i1 (stored as byte)
            Type::Str => 8,    // pointer
            Type::Void => 0,
            Type::List(_) => 8,  // pointer
            Type::Dict(_, _) => 8,  // pointer
            Type::Array(inner, size) => Self::type_size(inner) * (*size as usize),
            Type::Optional(_) => 8,  // pointer (nullable)
            Type::Custom(_) => 8,  // pointer to struct
            Type::Exception => 8,  // pointer
        }
    }

    /// Allocate memory for a variable and register it with JIT
    fn allocate_variable(&mut self, name: &str, ws_type: &Type) {
        // Don't re-allocate if already exists
        if self.variables.contains_key(name) {
            return;
        }

        let size = Self::type_size(ws_type);
        if size == 0 {
            return;  // Don't allocate void type
        }

        // Allocate zeroed memory
        let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };

        // Register with JIT
        self.jit.register_variable(name, ptr);

        // Store in our map
        self.variables.insert(name.to_string(), ReplVariable {
            ws_type: ws_type.clone(),
            ptr,
            size,
        });
    }

    /// Create a new REPL instance
    pub fn new() -> Result<Self, String> {
        // Leak the context to get 'static lifetime for JIT
        let context = Box::leak(Box::new(Context::create()));
        let jit = JitEngine::new(context)?;

        Ok(Repl {
            context,
            type_checker: TypeChecker::new(),
            variables: HashMap::new(),
            user_functions: HashMap::new(),
            functions: HashMap::new(),
            jit,
            input_buffer: String::new(),
        })
    }

    /// Run the REPL main loop
    pub fn run(&mut self) {
        use std::io;
        use std::os::unix::io::AsRawFd;

        println!("WadeScript REPL v0.1.0");
        println!("Type 'exit' or Ctrl+D to quit\n");

        // Check if stdin is a TTY
        let stdin_is_tty = unsafe { libc::isatty(io::stdin().as_raw_fd()) } != 0;

        if stdin_is_tty {
            self.run_interactive();
        } else {
            self.run_noninteractive();
        }

        println!("Goodbye!");
    }

    /// Run interactive mode with rustyline
    fn run_interactive(&mut self) {
        let mut rl = match DefaultEditor::new() {
            Ok(editor) => editor,
            Err(e) => {
                eprintln!("Failed to initialize editor: {}", e);
                return;
            }
        };

        loop {
            let prompt = if self.input_buffer.is_empty() {
                ">>> "
            } else {
                "... "
            };

            match rl.readline(prompt) {
                Ok(line) => {
                    if !self.process_line(&line, &mut rl) {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    // Ctrl+C - clear current input
                    self.input_buffer.clear();
                    println!();
                }
                Err(ReadlineError::Eof) => {
                    // Ctrl+D - exit
                    break;
                }
                Err(err) => {
                    eprintln!("Error reading input: {}", err);
                    break;
                }
            }
        }
    }

    /// Run non-interactive mode (piped input)
    fn run_noninteractive(&mut self) {
        use std::io::{self, BufRead};

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => {
                    if !self.process_line_simple(&line) {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    }

    /// Process a line in interactive mode
    fn process_line(&mut self, line: &str, rl: &mut DefaultEditor) -> bool {
        let trimmed = line.trim();

        // Check for exit command
        if trimmed == "exit" && self.input_buffer.is_empty() {
            return false;
        }

        // Append to buffer
        self.input_buffer.push_str(line);
        self.input_buffer.push('\n');

        // Check if input is complete
        if !self.is_complete(&self.input_buffer) {
            return true;
        }

        // Add to history
        let input = self.input_buffer.trim().to_string();
        if !input.is_empty() {
            let _ = rl.add_history_entry(&input);
        }

        // Evaluate the input
        if !input.is_empty() {
            match self.eval(&input) {
                Ok(Some(result)) => println!("{}", result),
                Ok(None) => {}
                Err(e) => eprintln!("\x1b[31mError:\x1b[0m {}", e),
            }
        }

        // Clear buffer
        self.input_buffer.clear();
        true
    }

    /// Process a line in non-interactive mode
    fn process_line_simple(&mut self, line: &str) -> bool {
        let trimmed = line.trim();

        // Check for exit command
        if trimmed == "exit" && self.input_buffer.is_empty() {
            return false;
        }

        // Append to buffer
        self.input_buffer.push_str(line);
        self.input_buffer.push('\n');

        // Check if input is complete
        if !self.is_complete(&self.input_buffer) {
            return true;
        }

        // Evaluate the input
        let input = self.input_buffer.trim().to_string();
        if !input.is_empty() {
            match self.eval(&input) {
                Ok(Some(result)) => println!("{}", result),
                Ok(None) => {}
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        // Clear buffer
        self.input_buffer.clear();
        true
    }

    /// Check if input is complete (balanced brackets)
    fn is_complete(&self, input: &str) -> bool {
        let mut brace_count = 0i32;
        let mut paren_count = 0i32;
        let mut bracket_count = 0i32;
        let mut in_string = false;
        let mut prev_char = '\0';

        for ch in input.chars() {
            if ch == '"' && prev_char != '\\' {
                in_string = !in_string;
            }

            if !in_string {
                match ch {
                    '{' => brace_count += 1,
                    '}' => brace_count -= 1,
                    '(' => paren_count += 1,
                    ')' => paren_count -= 1,
                    '[' => bracket_count += 1,
                    ']' => bracket_count -= 1,
                    _ => {}
                }
            }

            prev_char = ch;
        }

        brace_count == 0 && paren_count == 0 && bracket_count == 0 && !in_string
    }

    /// Extract variable declarations from statements
    fn extract_var_declarations(statements: &[Statement]) -> Vec<(String, Type)> {
        let mut vars = Vec::new();
        for stmt in statements {
            if let Statement::VarDecl { name, type_annotation, .. } = stmt {
                vars.push((name.clone(), type_annotation.clone()));
            }
        }
        vars
    }

    /// Evaluate a REPL input
    fn eval(&mut self, input: &str) -> Result<Option<String>, String> {
        // Parse the input
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        // Try to parse as a program (statements)
        let program = parser.parse();

        if program.statements.is_empty() {
            return Ok(None);
        }

        // Extract variable declarations from this input
        let new_vars = Self::extract_var_declarations(&program.statements);

        // Register existing persisted variables with type checker
        for (name, var) in &self.variables {
            self.type_checker.register_repl_variable(name, &var.ws_type);
        }

        // Allocate memory for new variables BEFORE compilation
        for (name, var_type) in &new_vars {
            self.allocate_variable(name, var_type);
            // Also register new variables with type checker
            self.type_checker.register_repl_variable(name, var_type);
        }

        // Generate unique entry function name
        let entry_name = self.jit.next_entry_name();

        // Wrap in function for compilation
        let wrapped_program = self.wrap_in_function(&program, &entry_name);

        // Type check the wrapped program
        self.type_checker.check_program(&wrapped_program)?;

        // Compile to LLVM IR
        let module = self.compile_repl_input_direct(&wrapped_program, &new_vars)?;

        // Add module to JIT
        self.jit.add_module(module)?;

        // Execute the entry function
        // CodeGen adds a "ws_" prefix to function names
        let mangled_name = format!("ws_{}", entry_name);
        unsafe {
            let entry_fn = self.jit.get_function_raw(&mangled_name);

            match entry_fn {
                Ok(func) => {
                    let result = func.call();
                    // For now, only return result if it's non-zero (indicates expression value)
                    // This is a simplified approach - we'll refine later
                    if result != 0 {
                        Ok(Some(result.to_string()))
                    } else {
                        Ok(None)
                    }
                }
                Err(e) => Err(format!("Failed to execute: {}", e)),
            }
        }
    }

    /// Compile a pre-wrapped REPL program to LLVM module
    fn compile_repl_input_direct(&mut self, program: &Program, new_vars: &[(String, Type)]) -> Result<Module<'static>, String> {
        let mut codegen = CodeGen::new(self.context, "repl_module", "<repl>");

        // Declare runtime functions
        codegen.declare_runtime_functions();

        // Declare all previously defined user functions as external
        for (name, func_info) in &self.user_functions {
            let param_types: Vec<Type> = func_info.params.iter().map(|(_, t)| t.clone()).collect();
            codegen.declare_external_function(name, &param_types, &func_info.return_type);
        }

        // Declare all existing persisted variables as external globals
        for (name, var) in &self.variables {
            codegen.declare_repl_variable(name, &var.ws_type);
        }

        // Declare new variables that will be created in this input
        for (name, var_type) in new_vars {
            if !self.variables.contains_key(name) {
                codegen.declare_repl_variable(name, var_type);
            }
        }

        // Track any new function definitions in this input
        for stmt in &program.statements {
            if let Statement::FunctionDef { name, params, return_type, .. } = stmt {
                // Don't track the entry function (starts with __repl_entry_)
                if !name.starts_with("__repl_entry_") {
                    self.user_functions.insert(name.clone(), UserFunction {
                        params: params.iter().map(|p| (p.name.clone(), p.param_type.clone())).collect(),
                        return_type: return_type.clone(),
                    });
                }
            }
        }

        // Compile all statements (the wrapped function)
        for stmt in &program.statements {
            codegen.compile_statement_repl(stmt)?;
        }

        Ok(codegen.take_module())
    }

    /// Wrap REPL statements in a function for execution
    /// Function/class definitions stay at module level, other statements go in entry function
    fn wrap_in_function(&self, program: &Program, entry_name: &str) -> Program {
        let mut module_level: Vec<Statement> = Vec::new();
        let mut body: Vec<Statement> = Vec::new();

        // Separate module-level definitions from executable statements
        for stmt in &program.statements {
            match stmt {
                Statement::FunctionDef { .. } | Statement::ClassDef { .. } => {
                    module_level.push(stmt.clone());
                }
                _ => {
                    body.push(stmt.clone());
                }
            }
        }

        // Add a return 0 at the end if not already present
        let has_return = body.iter().any(|s| matches!(s, Statement::Return(_)));
        if !has_return {
            body.push(Statement::Return(Some(Expression::IntLiteral(0))));
        }

        // Create entry function with executable statements
        let main_fn = Statement::FunctionDef {
            name: entry_name.to_string(),
            params: vec![],
            return_type: Type::Int,
            body,
        };

        // Add module-level definitions first, then entry function
        let mut all_statements = module_level;
        all_statements.push(main_fn);

        Program {
            statements: all_statements,
            modules: std::collections::HashMap::new(),
        }
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new().expect("Failed to create REPL")
    }
}
