//! JIT Engine for WadeScript REPL
//!
//! Provides LLVM JIT compilation support using inkwell's ExecutionEngine.

use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::OptimizationLevel;

/// JIT Engine wrapper that manages LLVM execution engine and runtime symbols
pub struct JitEngine<'ctx> {
    #[allow(dead_code)]
    context: &'ctx Context,
    execution_engine: ExecutionEngine<'ctx>,
    input_counter: usize,
}

impl<'ctx> JitEngine<'ctx> {
    /// Create a new JIT engine with runtime symbols registered
    pub fn new(context: &'ctx Context) -> Result<Self, String> {
        // Initialize LLVM targets for JIT
        Target::initialize_native(&InitializationConfig::default())
            .map_err(|e| format!("Failed to initialize native target: {}", e))?;

        // Create a dummy module to initialize the execution engine
        let module = context.create_module("__jit_init__");

        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::Default)
            .map_err(|e| format!("Failed to create JIT execution engine: {:?}", e))?;

        let jit = JitEngine {
            context,
            execution_engine,
            input_counter: 0,
        };

        // Register runtime symbols using linkage
        jit.register_runtime_symbols_direct();

        Ok(jit)
    }

    /// Register runtime symbols using LLVM's global symbol table
    /// Uses the centralized registry in runtime_symbols.rs to ensure
    /// JIT stays in sync with compiled mode.
    fn register_runtime_symbols_direct(&self) {
        use crate::runtime_symbols::get_runtime_symbols;
        use std::ffi::CString;

        // Get all symbols from the centralized registry
        let symbols = get_runtime_symbols();

        for symbol in symbols {
            let cname = CString::new(symbol.name).unwrap();
            unsafe {
                extern "C" {
                    fn LLVMAddSymbol(symbolName: *const std::os::raw::c_char, symbolValue: *mut std::ffi::c_void);
                }
                LLVMAddSymbol(cname.as_ptr(), symbol.addr as *mut std::ffi::c_void);
            }
        }
    }

    /// Add a module to the JIT engine and compile it
    pub fn add_module(&self, module: Module<'ctx>) -> Result<(), String> {
        self.execution_engine.add_module(&module)
            .map_err(|_| "Failed to add module to execution engine".to_string())
    }

    /// Get a JIT-compiled function by name
    pub unsafe fn get_function_raw(&self, name: &str) -> Result<JitFunction<'ctx, ReplEntryFn>, String> {
        self.execution_engine.get_function::<ReplEntryFn>(name)
            .map_err(|e| format!("Failed to get function '{}': {:?}", name, e))
    }

    /// Generate a unique name for a REPL entry function
    pub fn next_entry_name(&mut self) -> String {
        self.input_counter += 1;
        format!("__repl_entry_{}__", self.input_counter)
    }

    /// Register a persistent variable's address with the JIT
    pub fn register_variable(&self, name: &str, addr: *mut u8) {
        use std::ffi::CString;
        let symbol_name = format!("__repl_var_{}__", name);
        let cname = CString::new(symbol_name).unwrap();
        unsafe {
            extern "C" {
                fn LLVMAddSymbol(symbolName: *const std::os::raw::c_char, symbolValue: *mut std::ffi::c_void);
            }
            LLVMAddSymbol(cname.as_ptr(), addr as *mut std::ffi::c_void);
        }
    }

    /// Get the LLVM context (reserved for future use)
    #[allow(dead_code)]
    pub fn context(&self) -> &'ctx Context {
        self.context
    }
}

/// Type alias for REPL entry functions (no args, returns i64)
pub type ReplEntryFn = unsafe extern "C" fn() -> i64;

/// Type alias for REPL expression functions that return int (reserved for future use)
#[allow(dead_code)]
pub type ReplIntFn = unsafe extern "C" fn() -> i64;

/// Type alias for REPL expression functions that return float (reserved for future use)
#[allow(dead_code)]
pub type ReplFloatFn = unsafe extern "C" fn() -> f64;

/// Type alias for REPL expression functions that return bool (reserved for future use)
#[allow(dead_code)]
pub type ReplBoolFn = unsafe extern "C" fn() -> bool;

/// Type alias for REPL expression functions that return string pointer (reserved for future use)
#[allow(dead_code)]
pub type ReplStrFn = unsafe extern "C" fn() -> *const u8;
