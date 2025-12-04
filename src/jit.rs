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
    fn register_runtime_symbols_direct(&self) {
        use crate::runtime::list::*;
        use crate::runtime::dict::*;
        use crate::runtime::string::*;
        use crate::runtime::rc::*;
        use crate::runtime::io::*;
        use crate::runtime::exceptions::*;
        use crate::runtime::{push_call_stack, pop_call_stack, runtime_error};
        use std::ffi::CString;

        // Helper to add a symbol to LLVM's global symbol table
        fn add_symbol(name: &str, addr: usize) {
            let cname = CString::new(name).unwrap();
            unsafe {
                // Use LLVM's C API directly
                extern "C" {
                    fn LLVMAddSymbol(symbolName: *const std::os::raw::c_char, symbolValue: *mut std::ffi::c_void);
                }
                LLVMAddSymbol(cname.as_ptr(), addr as *mut std::ffi::c_void);
            }
        }

        // List operations
        add_symbol("list_get_i64", list_get_i64 as usize);
        add_symbol("list_push_i64", list_push_i64 as usize);
        add_symbol("list_pop_i64", list_pop_i64 as usize);
        add_symbol("list_set_i64", list_set_i64 as usize);

        // Dict operations
        add_symbol("dict_create", dict_create as usize);
        add_symbol("dict_set", dict_set as usize);
        add_symbol("dict_get", dict_get as usize);
        add_symbol("dict_has", dict_has as usize);

        // String operations
        add_symbol("str_length", str_length as usize);
        add_symbol("str_upper", str_upper as usize);
        add_symbol("str_lower", str_lower as usize);
        add_symbol("str_contains", str_contains as usize);
        add_symbol("str_char_at", str_char_at as usize);

        // RC operations
        add_symbol("rc_alloc", rc_alloc as usize);
        add_symbol("rc_retain", rc_retain as usize);
        add_symbol("rc_release", rc_release as usize);
        add_symbol("rc_get_count", rc_get_count as usize);
        add_symbol("rc_is_valid", rc_is_valid as usize);

        // File I/O operations
        add_symbol("file_open", file_open as usize);
        add_symbol("file_read", file_read as usize);
        add_symbol("file_read_line", file_read_line as usize);
        add_symbol("file_write", file_write as usize);
        add_symbol("file_close", file_close as usize);
        add_symbol("file_exists", file_exists as usize);

        // Exception handling
        add_symbol("exception_create", exception_create as usize);
        add_symbol("exception_get_current", exception_get_current as usize);
        add_symbol("exception_set_current", exception_set_current as usize);
        add_symbol("exception_clear", exception_clear as usize);
        add_symbol("exception_get_type", exception_get_type as usize);
        add_symbol("exception_get_message", exception_get_message as usize);
        add_symbol("exception_matches", exception_matches as usize);
        add_symbol("exception_push_handler", exception_push_handler as usize);
        add_symbol("exception_pop_handler", exception_pop_handler as usize);
        add_symbol("exception_raise", exception_raise as usize);

        // Call stack functions
        add_symbol("push_call_stack", push_call_stack as usize);
        add_symbol("pop_call_stack", pop_call_stack as usize);
        add_symbol("runtime_error", runtime_error as usize);

        // Standard C library functions (these should already be available but add anyway)
        add_symbol("printf", libc::printf as usize);
        add_symbol("malloc", libc::malloc as usize);
        add_symbol("free", libc::free as usize);
        add_symbol("memcpy", libc::memcpy as usize);
        add_symbol("strlen", libc::strlen as usize);
        add_symbol("exit", libc::exit as usize);
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
