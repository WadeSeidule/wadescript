use crate::ast::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, StructType};
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, PointerValue};
use inkwell::basic_block::BasicBlock;
use inkwell::{AddressSpace, IntPredicate, FloatPredicate};
use inkwell::debug_info::{AsDIScope, DICompileUnit, DIFlagsConstants, DWARFEmissionKind, DWARFSourceLanguage, DebugInfoBuilder, DISubprogram};
use std::collections::HashMap;

// Loop context for break/continue
struct LoopContext<'ctx> {
    continue_block: BasicBlock<'ctx>,
    break_block: BasicBlock<'ctx>,
}

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>, Type)>, // Added AST Type
    functions: HashMap<String, FunctionValue<'ctx>>,
    current_function: Option<FunctionValue<'ctx>>,
    class_types: HashMap<String, StructType<'ctx>>,
    class_fields: HashMap<String, Vec<String>>, // class_name -> field names in order
    current_class: Option<String>, // Track current class being compiled
    loop_stack: Vec<LoopContext<'ctx>>, // Stack of loop contexts for break/continue
    // Debug info
    debug_builder: DebugInfoBuilder<'ctx>,
    compile_unit: DICompileUnit<'ctx>,
    source_file: String,
    current_debug_scope: Option<DISubprogram<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str, source_file: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        // Create debug info builder
        let (debug_builder, compile_unit) = module.create_debug_info_builder(
            true, // allow_unresolved
            DWARFSourceLanguage::C, // closest to WadeScript
            source_file,
            ".",
            "WadeScript Compiler",
            false, // is_optimized
            "",
            0,
            "",
            DWARFEmissionKind::Full,
            0,
            false,
            false,
            "",
            "",
        );

        CodeGen {
            context,
            module,
            builder,
            variables: HashMap::new(),
            functions: HashMap::new(),
            current_function: None,
            class_types: HashMap::new(),
            class_fields: HashMap::new(),
            current_class: None,
            loop_stack: Vec::new(),
            debug_builder,
            compile_unit,
            source_file: source_file.to_string(),
            current_debug_scope: None,
        }
    }

    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    fn get_llvm_type(&self, ws_type: &Type) -> BasicTypeEnum<'ctx> {
        match ws_type {
            Type::Int => self.context.i64_type().as_basic_type_enum(),
            Type::Float => self.context.f64_type().as_basic_type_enum(),
            Type::Bool => self.context.bool_type().as_basic_type_enum(),
            Type::Str => self
                .context
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum(),
            Type::Void => self.context.i64_type().as_basic_type_enum(),
            Type::Array(elem_type, size) => {
                let elem_llvm_type = self.get_llvm_type(elem_type);
                elem_llvm_type
                    .array_type(*size as u32)
                    .as_basic_type_enum()
            }
            Type::List(_) | Type::Dict(_, _) => {
                // For now, represent lists and dicts as opaque pointers
                // Full implementation would need runtime struct definitions
                self.context
                    .ptr_type(AddressSpace::default())
                    .as_basic_type_enum()
            }
            Type::Exception => {
                // Exception object is a pointer to a struct
                self.context
                    .ptr_type(AddressSpace::default())
                    .as_basic_type_enum()
            }
            Type::Custom(_) => self
                .context
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum(),
        }
    }

    // Helper: Check if a type needs reference counting
    fn is_rc_type(&self, ws_type: &Type) -> bool {
        // Note: Str excluded for now because string literals are global constants
        // We'll add proper string RC later (need to distinguish literals from allocated strings)
        matches!(ws_type, Type::List(_) | Type::Dict(_, _) | Type::Custom(_))
    }

    // Inline RC retain: increment reference count
    fn build_rc_retain_inline(&self, ptr: PointerValue<'ctx>) {
        let i64_type = self.context.i64_type();
        let i8_type = self.context.i8_type();

        // ptr points to object data
        // header is 8 bytes before: [ref_count: i64][data...]

        // Get header pointer: ptr - 8
        let minus_8 = i64_type.const_int((-8i64) as u64, false);
        let header = unsafe {
            self.builder.build_gep(
                i8_type,
                ptr,
                &[minus_8],
                "rc_header"
            ).unwrap()
        };

        // Load current count
        let count = self.builder.build_load(
            i64_type,
            header,
            "ref_count"
        ).unwrap().into_int_value();

        // Increment
        let new_count = self.builder.build_int_add(
            count,
            i64_type.const_int(1, false),
            "new_count"
        ).unwrap();

        // Store back
        self.builder.build_store(header, new_count).unwrap();
    }

    // Release all RC variables in current scope
    fn release_scope_variables(&self) {
        for (_name, (ptr, var_type, ast_type)) in &self.variables {
            if self.is_rc_type(ast_type) {
                // Load the pointer value
                let val = self.builder.build_load(*var_type, *ptr, "scope_val").unwrap();
                if val.is_pointer_value() {
                    let obj_ptr = val.into_pointer_value();
                    // Check if not null before releasing
                    let is_null = self.builder.build_is_null(obj_ptr, "is_null").unwrap();
                    let function = self.current_function.unwrap();
                    let release_block = self.context.append_basic_block(function, "scope_release");
                    let continue_block = self.context.append_basic_block(function, "scope_continue");

                    self.builder.build_conditional_branch(is_null, continue_block, release_block).unwrap();

                    self.builder.position_at_end(release_block);
                    self.build_rc_release_inline(obj_ptr);
                    self.builder.build_unconditional_branch(continue_block).unwrap();

                    self.builder.position_at_end(continue_block);
                }
            }
        }
    }

    // Inline RC release: decrement reference count and free if zero
    fn build_rc_release_inline(&self, ptr: PointerValue<'ctx>) {
        let i64_type = self.context.i64_type();
        let i8_type = self.context.i8_type();
        let function = self.current_function.unwrap();

        // Get header
        let minus_8 = i64_type.const_int((-8i64) as u64, false);
        let header = unsafe {
            self.builder.build_gep(i8_type, ptr, &[minus_8], "rc_header").unwrap()
        };

        // Load count
        let count = self.builder.build_load(i64_type, header, "ref_count")
            .unwrap().into_int_value();

        // Decrement
        let new_count = self.builder.build_int_sub(
            count,
            i64_type.const_int(1, false),
            "new_count"
        ).unwrap();

        // Store
        self.builder.build_store(header, new_count).unwrap();

        // Check if we hit zero
        let is_zero = self.builder.build_int_compare(
            IntPredicate::EQ,
            new_count,
            i64_type.const_zero(),
            "is_zero"
        ).unwrap();

        let free_block = self.context.append_basic_block(function, "rc_free");
        let continue_block = self.context.append_basic_block(function, "rc_continue");

        self.builder.build_conditional_branch(is_zero, free_block, continue_block).unwrap();

        // Free block: deallocate memory
        self.builder.position_at_end(free_block);
        let free_fn = self.functions.get("free").unwrap();
        self.builder.build_call(*free_fn, &[header.into()], "").unwrap();
        self.builder.build_unconditional_branch(continue_block).unwrap();

        // Continue
        self.builder.position_at_end(continue_block);
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<(), String> {
        self.declare_printf();
        self.declare_memory_functions();
        self.declare_builtin_functions();
        self.declare_list_functions();
        self.declare_dict_functions();
        self.declare_string_functions();
        self.declare_runtime_error_functions();

        for statement in &program.statements {
            self.compile_statement(statement)?;
        }

        // Finalize debug info
        self.debug_builder.finalize();

        Ok(())
    }

    fn declare_printf(&mut self) {
        let i32_type = self.context.i32_type();
        let str_type = self.context.ptr_type(AddressSpace::default());

        let printf_type = i32_type.fn_type(&[str_type.into()], true);
        self.module.add_function("printf", printf_type, None);
    }

    fn declare_memory_functions(&mut self) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        // malloc(size) -> ptr
        let malloc_type = ptr_type.fn_type(&[i64_type.into()], false);
        let malloc_fn = self.module.add_function("malloc", malloc_type, None);
        self.functions.insert("malloc".to_string(), malloc_fn);

        // free(ptr) -> void
        let free_type = self.context.void_type().fn_type(&[ptr_type.into()], false);
        let free_fn = self.module.add_function("free", free_type, None);
        self.functions.insert("free".to_string(), free_fn);

        // realloc(ptr, size) -> ptr
        let realloc_type = ptr_type.fn_type(&[ptr_type.into(), i64_type.into()], false);
        let realloc_fn = self.module.add_function("realloc", realloc_type, None);
        self.functions.insert("realloc".to_string(), realloc_fn);

        // memcpy(dest, src, size) -> ptr
        let memcpy_type = ptr_type.fn_type(&[ptr_type.into(), ptr_type.into(), i64_type.into()], false);
        let memcpy_fn = self.module.add_function("memcpy", memcpy_type, None);
        self.functions.insert("memcpy".to_string(), memcpy_fn);

        // strlen(str) -> i64
        let strlen_type = i64_type.fn_type(&[ptr_type.into()], false);
        let strlen_fn = self.module.add_function("strlen", strlen_type, None);
        self.functions.insert("strlen".to_string(), strlen_fn);

        // strcpy(dest, src) -> ptr
        let strcpy_type = ptr_type.fn_type(&[ptr_type.into(), ptr_type.into()], false);
        let strcpy_fn = self.module.add_function("strcpy", strcpy_type, None);
        self.functions.insert("strcpy".to_string(), strcpy_fn);

        // strcat(dest, src) -> ptr
        let strcat_type = ptr_type.fn_type(&[ptr_type.into(), ptr_type.into()], false);
        let strcat_fn = self.module.add_function("strcat", strcat_type, None);
        self.functions.insert("strcat".to_string(), strcat_fn);

        // sprintf(dest, format, ...) -> i32 (variadic)
        let i32_type = self.context.i32_type();
        let sprintf_type = i32_type.fn_type(&[ptr_type.into(), ptr_type.into()], true);
        let sprintf_fn = self.module.add_function("sprintf", sprintf_type, None);
        self.functions.insert("sprintf".to_string(), sprintf_fn);

        // strcmp(str1, str2) -> i32
        let strcmp_type = i32_type.fn_type(&[ptr_type.into(), ptr_type.into()], false);
        let strcmp_fn = self.module.add_function("strcmp", strcmp_type, None);
        self.functions.insert("strcmp".to_string(), strcmp_fn);

        // RC functions
        // rc_alloc(size) -> ptr
        let rc_alloc_type = ptr_type.fn_type(&[i64_type.into()], false);
        let rc_alloc_fn = self.module.add_function("rc_alloc", rc_alloc_type, None);
        self.functions.insert("rc_alloc".to_string(), rc_alloc_fn);

        // rc_retain(ptr) -> void
        let rc_retain_type = self.context.void_type().fn_type(&[ptr_type.into()], false);
        let rc_retain_fn = self.module.add_function("rc_retain", rc_retain_type, None);
        self.functions.insert("rc_retain".to_string(), rc_retain_fn);

        // rc_release(ptr) -> void
        let rc_release_type = self.context.void_type().fn_type(&[ptr_type.into()], false);
        let rc_release_fn = self.module.add_function("rc_release", rc_release_type, None);
        self.functions.insert("rc_release".to_string(), rc_release_fn);
    }

    fn declare_builtin_functions(&mut self) {
        let i64_type = self.context.i64_type();
        let f64_type = self.context.f64_type();
        let str_type = self.context.ptr_type(AddressSpace::default());
        let void_type = self.context.void_type();

        // print_int(int) -> void
        let print_int_type = void_type.fn_type(&[i64_type.into()], false);
        let print_int_fn = self.module.add_function("print_int", print_int_type, None);
        let entry = self.context.append_basic_block(print_int_fn, "entry");
        self.builder.position_at_end(entry);
        let format_str = self.builder.build_global_string_ptr("%lld\n", "int_fmt").unwrap();
        let printf = self.module.get_function("printf").unwrap();
        let arg = print_int_fn.get_nth_param(0).unwrap();
        self.builder.build_call(printf, &[format_str.as_pointer_value().into(), arg.into()], "").unwrap();
        self.builder.build_return(None).unwrap();
        self.functions.insert("print_int".to_string(), print_int_fn);

        // print_float(float) -> void
        let print_float_type = void_type.fn_type(&[f64_type.into()], false);
        let print_float_fn = self.module.add_function("print_float", print_float_type, None);
        let entry = self.context.append_basic_block(print_float_fn, "entry");
        self.builder.position_at_end(entry);
        let format_str = self.builder.build_global_string_ptr("%f\n", "float_fmt").unwrap();
        let arg = print_float_fn.get_nth_param(0).unwrap();
        self.builder.build_call(printf, &[format_str.as_pointer_value().into(), arg.into()], "").unwrap();
        self.builder.build_return(None).unwrap();
        self.functions.insert("print_float".to_string(), print_float_fn);

        // print_str(str) -> void
        let print_str_type = void_type.fn_type(&[str_type.into()], false);
        let print_str_fn = self.module.add_function("print_str", print_str_type, None);
        let entry = self.context.append_basic_block(print_str_fn, "entry");
        self.builder.position_at_end(entry);
        let format_str = self.builder.build_global_string_ptr("%s\n", "str_fmt").unwrap();
        let arg = print_str_fn.get_nth_param(0).unwrap();
        self.builder.build_call(printf, &[format_str.as_pointer_value().into(), arg.into()], "").unwrap();
        self.builder.build_return(None).unwrap();
        self.functions.insert("print_str".to_string(), print_str_fn);

        // print_bool(bool) -> void
        let print_bool_type = void_type.fn_type(&[self.context.bool_type().into()], false);
        let print_bool_fn = self.module.add_function("print_bool", print_bool_type, None);
        let entry = self.context.append_basic_block(print_bool_fn, "entry");
        self.builder.position_at_end(entry);

        let arg = print_bool_fn.get_nth_param(0).unwrap().into_int_value();
        let then_block = self.context.append_basic_block(print_bool_fn, "then");
        let else_block = self.context.append_basic_block(print_bool_fn, "else");
        let merge_block = self.context.append_basic_block(print_bool_fn, "merge");

        self.builder.build_conditional_branch(arg, then_block, else_block).unwrap();

        self.builder.position_at_end(then_block);
        let true_str = self.builder.build_global_string_ptr("True\n", "true_str").unwrap();
        self.builder.build_call(printf, &[true_str.as_pointer_value().into()], "").unwrap();
        self.builder.build_unconditional_branch(merge_block).unwrap();

        self.builder.position_at_end(else_block);
        let false_str = self.builder.build_global_string_ptr("False\n", "false_str").unwrap();
        self.builder.build_call(printf, &[false_str.as_pointer_value().into()], "").unwrap();
        self.builder.build_unconditional_branch(merge_block).unwrap();

        self.builder.position_at_end(merge_block);
        self.builder.build_return(None).unwrap();
        self.functions.insert("print_bool".to_string(), print_bool_fn);
    }

    fn declare_list_functions(&mut self) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();
        let void_type = self.context.void_type();

        // List structure in memory: { ptr data, i64 length, i64 capacity }
        // Total size: 24 bytes (8 + 8 + 8)

        // list_create_i64() -> ptr (creates empty list for i64 elements)
        let list_create_type = ptr_type.fn_type(&[], false);
        let list_create_fn = self.module.add_function("list_create_i64", list_create_type, None);
        let entry = self.context.append_basic_block(list_create_fn, "entry");
        self.builder.position_at_end(entry);

        // Allocate list struct (24 bytes) with RC
        let rc_alloc = self.functions.get("rc_alloc").unwrap();
        let struct_size = self.context.i64_type().const_int(24, false);
        let list_ptr = self.builder
            .build_call(*rc_alloc, &[struct_size.into()], "list_ptr")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        // Initialize: data = NULL, length = 0, capacity = 0
        let zero = i64_type.const_zero();
        let null_ptr = ptr_type.const_null();

        // Store data pointer (offset 0)
        let data_ptr = self.builder.build_pointer_cast(list_ptr, ptr_type, "").unwrap();
        self.builder.build_store(data_ptr, null_ptr).unwrap();

        // Store length (offset 8)
        let list_as_i64_ptr = self.builder.build_pointer_cast(list_ptr, ptr_type, "").unwrap();
        let length_ptr = unsafe {
            self.builder.build_gep(
                ptr_type,
                list_as_i64_ptr,
                &[self.context.i64_type().const_int(1, false)],
                "length_ptr"
            ).unwrap()
        };
        let length_ptr_typed = self.builder.build_pointer_cast(length_ptr, ptr_type, "").unwrap();
        self.builder.build_store(length_ptr_typed, zero).unwrap();

        // Store capacity (offset 16)
        let capacity_ptr = unsafe {
            self.builder.build_gep(
                ptr_type,
                list_as_i64_ptr,
                &[self.context.i64_type().const_int(2, false)],
                "capacity_ptr"
            ).unwrap()
        };
        let capacity_ptr_typed = self.builder.build_pointer_cast(capacity_ptr, ptr_type, "").unwrap();
        self.builder.build_store(capacity_ptr_typed, zero).unwrap();

        self.builder.build_return(Some(&list_ptr)).unwrap();
        self.functions.insert("list_create_i64".to_string(), list_create_fn);

        // list_push_i64(list_ptr, value) -> void
        let list_push_type = void_type.fn_type(&[ptr_type.into(), i64_type.into()], false);
        let list_push_fn = self.module.add_function("list_push_i64", list_push_type, None);
        self.functions.insert("list_push_i64".to_string(), list_push_fn);

        // list_get_i64(list_ptr, index) -> i64
        let list_get_type = i64_type.fn_type(&[ptr_type.into(), i64_type.into()], false);
        let list_get_fn = self.module.add_function("list_get_i64", list_get_type, None);
        self.functions.insert("list_get_i64".to_string(), list_get_fn);

        // list_set_i64(list_ptr, index, value) -> void
        let list_set_type = void_type.fn_type(&[ptr_type.into(), i64_type.into(), i64_type.into()], false);
        let list_set_fn = self.module.add_function("list_set_i64", list_set_type, None);
        self.functions.insert("list_set_i64".to_string(), list_set_fn);

        // list_pop_i64(list_ptr) -> i64
        let list_pop_type = i64_type.fn_type(&[ptr_type.into()], false);
        let list_pop_fn = self.module.add_function("list_pop_i64", list_pop_type, None);
        self.functions.insert("list_pop_i64".to_string(), list_pop_fn);

        // list_length(list_ptr) -> i64
        let list_length_type = i64_type.fn_type(&[ptr_type.into()], false);
        let list_length_fn = self.module.add_function("list_length", list_length_type, None);
        let entry = self.context.append_basic_block(list_length_fn, "entry");
        self.builder.position_at_end(entry);

        let list_arg = list_length_fn.get_nth_param(0).unwrap().into_pointer_value();

        // Load length from offset 8
        let list_as_i64_ptr = self.builder.build_pointer_cast(list_arg, ptr_type, "").unwrap();
        let length_ptr = unsafe {
            self.builder.build_gep(
                ptr_type,
                list_as_i64_ptr,
                &[self.context.i64_type().const_int(1, false)],
                "length_ptr"
            ).unwrap()
        };
        let length = self.builder.build_load(i64_type, length_ptr, "length").unwrap();

        self.builder.build_return(Some(&length)).unwrap();
        self.functions.insert("list_length".to_string(), list_length_fn);
    }

    fn declare_dict_functions(&mut self) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();
        let i32_type = self.context.i32_type();
        let void_type = self.context.void_type();

        // Dict structure in memory: { ptr buckets, i64 capacity, i64 length }
        // Uses hash table with separate chaining for collisions
        // Total size: 24 bytes (8 + 8 + 8)

        // dict_create() -> ptr (creates empty hash table dict)
        let dict_create_type = ptr_type.fn_type(&[], false);
        let dict_create_fn = self.module.add_function("dict_create", dict_create_type, None);
        self.functions.insert("dict_create".to_string(), dict_create_fn);

        // dict_set(dict_ptr, key_str, value_int) -> void
        let dict_set_type = void_type.fn_type(&[ptr_type.into(), ptr_type.into(), i64_type.into()], false);
        let dict_set_fn = self.module.add_function("dict_set", dict_set_type, None);
        self.functions.insert("dict_set".to_string(), dict_set_fn);

        // dict_get(dict_ptr, key_str) -> i64
        let dict_get_type = i64_type.fn_type(&[ptr_type.into(), ptr_type.into()], false);
        let dict_get_fn = self.module.add_function("dict_get", dict_get_type, None);
        self.functions.insert("dict_get".to_string(), dict_get_fn);

        // dict_has(dict_ptr, key_str) -> i32 (returns 1 if exists, 0 otherwise)
        let dict_has_type = i32_type.fn_type(&[ptr_type.into(), ptr_type.into()], false);
        let dict_has_fn = self.module.add_function("dict_has", dict_has_type, None);
        self.functions.insert("dict_has".to_string(), dict_has_fn);

        // dict_length(dict_ptr) -> i64
        let dict_length_type = i64_type.fn_type(&[ptr_type.into()], false);
        let dict_length_fn = self.module.add_function("dict_length", dict_length_type, None);
        let entry = self.context.append_basic_block(dict_length_fn, "entry");
        self.builder.position_at_end(entry);

        let dict_arg = dict_length_fn.get_nth_param(0).unwrap().into_pointer_value();

        // Load length from offset 8
        let dict_as_ptr = self.builder.build_pointer_cast(dict_arg, ptr_type, "").unwrap();
        let length_ptr = unsafe {
            self.builder.build_gep(
                ptr_type,
                dict_as_ptr,
                &[i64_type.const_int(1, false)],
                "length_ptr"
            ).unwrap()
        };
        let length = self.builder.build_load(i64_type, length_ptr, "length").unwrap();

        self.builder.build_return(Some(&length)).unwrap();
        self.functions.insert("dict_length".to_string(), dict_length_fn);
    }

    fn declare_string_functions(&mut self) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();
        let i32_type = self.context.i32_type();

        // str_length(str_ptr) -> i64
        let str_length_type = i64_type.fn_type(&[ptr_type.into()], false);
        let str_length_fn = self.module.add_function("str_length", str_length_type, None);
        self.functions.insert("str_length".to_string(), str_length_fn);

        // str_upper(str_ptr) -> ptr (returns new string)
        let str_upper_type = ptr_type.fn_type(&[ptr_type.into()], false);
        let str_upper_fn = self.module.add_function("str_upper", str_upper_type, None);
        self.functions.insert("str_upper".to_string(), str_upper_fn);

        // str_lower(str_ptr) -> ptr (returns new string)
        let str_lower_type = ptr_type.fn_type(&[ptr_type.into()], false);
        let str_lower_fn = self.module.add_function("str_lower", str_lower_type, None);
        self.functions.insert("str_lower".to_string(), str_lower_fn);

        // str_contains(str_ptr, substring_ptr) -> i32
        let str_contains_type = i32_type.fn_type(&[ptr_type.into(), ptr_type.into()], false);
        let str_contains_fn = self.module.add_function("str_contains", str_contains_type, None);
        self.functions.insert("str_contains".to_string(), str_contains_fn);

        // str_char_at(str_ptr, index) -> ptr (returns single-char string)
        let str_char_at_type = ptr_type.fn_type(&[ptr_type.into(), i64_type.into()], false);
        let str_char_at_fn = self.module.add_function("str_char_at", str_char_at_type, None);
        self.functions.insert("str_char_at".to_string(), str_char_at_fn);
    }

    fn declare_runtime_error_functions(&mut self) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let void_type = self.context.void_type();
        let i64_type = self.context.i64_type();
        let i32_type = self.context.i32_type();

        // push_call_stack(func_name_ptr) -> void
        let push_call_stack_type = void_type.fn_type(&[ptr_type.into()], false);
        let push_call_stack_fn = self.module.add_function("push_call_stack", push_call_stack_type, None);
        self.functions.insert("push_call_stack".to_string(), push_call_stack_fn);

        // pop_call_stack() -> void
        let pop_call_stack_type = void_type.fn_type(&[], false);
        let pop_call_stack_fn = self.module.add_function("pop_call_stack", pop_call_stack_type, None);
        self.functions.insert("pop_call_stack".to_string(), pop_call_stack_fn);

        // exception_raise(type, message, file, line) -> noreturn
        let exception_raise_type = void_type.fn_type(
            &[ptr_type.into(), ptr_type.into(), ptr_type.into(), i64_type.into()],
            false
        );
        let exception_raise_fn = self.module.add_function("exception_raise", exception_raise_type, None);
        self.functions.insert("exception_raise".to_string(), exception_raise_fn);

        // exception_push_handler(jmp_buf) -> void
        let exception_push_handler_type = void_type.fn_type(&[ptr_type.into()], false);
        let exception_push_handler_fn = self.module.add_function("exception_push_handler", exception_push_handler_type, None);
        self.functions.insert("exception_push_handler".to_string(), exception_push_handler_fn);

        // exception_pop_handler() -> void
        let exception_pop_handler_type = void_type.fn_type(&[], false);
        let exception_pop_handler_fn = self.module.add_function("exception_pop_handler", exception_pop_handler_type, None);
        self.functions.insert("exception_pop_handler".to_string(), exception_pop_handler_fn);

        // exception_get_current() -> ptr
        let exception_get_current_type = ptr_type.fn_type(&[], false);
        let exception_get_current_fn = self.module.add_function("exception_get_current", exception_get_current_type, None);
        self.functions.insert("exception_get_current".to_string(), exception_get_current_fn);

        // exception_matches(exc, type) -> i32
        let exception_matches_type = i32_type.fn_type(&[ptr_type.into(), ptr_type.into()], false);
        let exception_matches_fn = self.module.add_function("exception_matches", exception_matches_type, None);
        self.functions.insert("exception_matches".to_string(), exception_matches_fn);

        // exception_clear() -> void
        let exception_clear_type = void_type.fn_type(&[], false);
        let exception_clear_fn = self.module.add_function("exception_clear", exception_clear_type, None);
        self.functions.insert("exception_clear".to_string(), exception_clear_fn);

        // setjmp(jmp_buf) -> i32
        let setjmp_type = i32_type.fn_type(&[ptr_type.into()], false);
        let setjmp_fn = self.module.add_function("setjmp", setjmp_type, None);
        self.functions.insert("setjmp".to_string(), setjmp_fn);
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::VarDecl {
                name,
                type_annotation,
                initializer,
            } => {
                let var_type = self.get_llvm_type(type_annotation);
                let alloca = self.builder.build_alloca(var_type, name).unwrap();

                if let Some(init_expr) = initializer {
                    let init_value = self.compile_expression(init_expr)?;

                    // For RC types, retain the initial value (it starts with ref_count=1 from allocation)
                    // No need to retain here since the allocation already gives us ownership

                    self.builder.build_store(alloca, init_value).unwrap();
                } else {
                    // Initialize RC types to null to prevent releasing garbage
                    if self.is_rc_type(type_annotation) {
                        let null_ptr = self.context.ptr_type(AddressSpace::default()).const_null();
                        self.builder.build_store(alloca, null_ptr).unwrap();
                    }
                }

                self.variables.insert(name.clone(), (alloca, var_type, type_annotation.clone()));
                Ok(())
            }

            Statement::FunctionDef {
                name,
                params,
                return_type,
                body,
            } => {
                let param_types: Vec<BasicMetadataTypeEnum> = params
                    .iter()
                    .map(|p| self.get_llvm_type(&p.param_type).into())
                    .collect();

                let fn_type = if *return_type == Type::Void {
                    self.context.void_type().fn_type(&param_types, false)
                } else {
                    let ret_type = self.get_llvm_type(return_type);
                    ret_type.fn_type(&param_types, false)
                };

                // Use qualified name for methods
                let function_key = if let Some(class_name) = &self.current_class {
                    format!("{}::{}", class_name, name)
                } else {
                    name.clone()
                };

                let function = self.module.add_function(name, fn_type, None);
                self.functions.insert(function_key, function);

                // Create debug info for this function
                let di_file = self.compile_unit.get_file();
                let di_func_type = self.debug_builder.create_subroutine_type(
                    di_file,
                    None, // return type (simplified for now)
                    &[], // parameter types (simplified for now)
                    inkwell::debug_info::DIFlags::PUBLIC,
                );

                let di_subprogram = self.debug_builder.create_function(
                    di_file.as_debug_info_scope(),
                    name,
                    None, // linkage name
                    di_file,
                    1, // line number (ideally would track this from AST)
                    di_func_type,
                    true, // is_local_to_unit
                    true, // is_definition
                    1, // scope_line
                    inkwell::debug_info::DIFlags::PUBLIC,
                    false, // is_optimized
                );

                // Attach debug info to the function
                function.set_subprogram(di_subprogram);

                // Save previous scope and set current scope to this function
                let saved_debug_scope = self.current_debug_scope;
                self.current_debug_scope = Some(di_subprogram);

                let entry = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(entry);

                // Push function name onto call stack for stack traces
                let func_name_str = self.builder.build_global_string_ptr(name, "func_name").unwrap();
                let push_call_stack_fn = *self.functions.get("push_call_stack").unwrap();
                self.builder.build_call(
                    push_call_stack_fn,
                    &[func_name_str.as_pointer_value().into()],
                    ""
                ).unwrap();

                let saved_variables = self.variables.clone();
                self.variables.clear();
                self.current_function = Some(function);

                for (i, param) in params.iter().enumerate() {
                    let param_value = function.get_nth_param(i as u32).unwrap();
                    let param_type = param_value.get_type();
                    let alloca = self
                        .builder
                        .build_alloca(param_type, &param.name)
                        .unwrap();
                    self.builder.build_store(alloca, param_value).unwrap();
                    self.variables.insert(param.name.clone(), (alloca, param_type, param.param_type.clone()));
                }

                let mut has_return = false;
                for stmt in body {
                    self.compile_statement(stmt)?;
                    if matches!(stmt, Statement::Return(_)) {
                        has_return = true;
                    }
                }

                if !has_return {
                    // Release all RC variables before returning
                    self.release_scope_variables();

                    // Pop function from call stack before returning
                    let pop_call_stack_fn = *self.functions.get("pop_call_stack").unwrap();
                    self.builder.build_call(pop_call_stack_fn, &[], "").unwrap();

                    if *return_type == Type::Void {
                        self.builder.build_return(None).unwrap();
                    } else {
                        let default_value = match return_type {
                            Type::Int => self.context.i64_type().const_zero().as_basic_value_enum(),
                            Type::Float => self.context.f64_type().const_zero().as_basic_value_enum(),
                            Type::Bool => self.context.bool_type().const_zero().as_basic_value_enum(),
                            _ => self
                                .context
                                .ptr_type(AddressSpace::default())
                                .const_null()
                                .as_basic_value_enum(),
                        };
                        self.builder.build_return(Some(&default_value)).unwrap();
                    }
                }

                self.variables = saved_variables;
                self.current_function = None;

                // Restore previous debug scope
                self.current_debug_scope = saved_debug_scope;

                Ok(())
            }

            Statement::ClassDef { name, fields, methods, .. } => {
                // Store field names in order
                let field_names: Vec<String> = fields.iter().map(|f| f.name.clone()).collect();
                self.class_fields.insert(name.clone(), field_names);

                // Create LLVM struct type for the class
                let field_types: Vec<BasicTypeEnum> = fields
                    .iter()
                    .map(|f| self.get_llvm_type(&f.field_type))
                    .collect();

                let struct_type = self.context.struct_type(&field_types, false);
                self.class_types.insert(name.clone(), struct_type);

                // Set current class context for method compilation
                self.current_class = Some(name.clone());

                // Compile methods first (so init exists before constructor calls it)
                for method in methods {
                    self.compile_statement(method)?;
                }

                // Clear class context
                self.current_class = None;

                // Generate constructor function (after methods are compiled)
                self.generate_constructor(name, fields)?;

                Ok(())
            }

            Statement::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => {
                let cond_value = self.compile_expression(condition)?;
                let cond_bool = if cond_value.is_int_value() {
                    cond_value.into_int_value()
                } else {
                    return Err("Condition must be a boolean".to_string());
                };

                let function = self
                    .current_function
                    .ok_or("If statement outside of function")?;

                let then_block = self.context.append_basic_block(function, "then");
                let merge_block = self.context.append_basic_block(function, "ifcont");

                if elif_branches.is_empty() && else_branch.is_none() {
                    self.builder
                        .build_conditional_branch(cond_bool, then_block, merge_block)
                        .unwrap();

                    self.builder.position_at_end(then_block);
                    for stmt in then_branch {
                        self.compile_statement(stmt)?;
                    }
                    if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                        self.builder.build_unconditional_branch(merge_block).unwrap();
                    }

                    self.builder.position_at_end(merge_block);
                } else {
                    let else_block = self.context.append_basic_block(function, "else");

                    self.builder
                        .build_conditional_branch(cond_bool, then_block, else_block)
                        .unwrap();

                    self.builder.position_at_end(then_block);
                    for stmt in then_branch {
                        self.compile_statement(stmt)?;
                    }
                    if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                        self.builder.build_unconditional_branch(merge_block).unwrap();
                    }

                    self.builder.position_at_end(else_block);

                    if !elif_branches.is_empty() {
                        for (elif_cond, elif_body) in elif_branches {
                            let elif_cond_value = self.compile_expression(elif_cond)?;
                            let elif_cond_bool = elif_cond_value.into_int_value();

                            let elif_then = self.context.append_basic_block(function, "elif_then");
                            let elif_else = self.context.append_basic_block(function, "elif_else");

                            self.builder
                                .build_conditional_branch(elif_cond_bool, elif_then, elif_else)
                                .unwrap();

                            self.builder.position_at_end(elif_then);
                            for stmt in elif_body {
                                self.compile_statement(stmt)?;
                            }
                            if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                                self.builder.build_unconditional_branch(merge_block).unwrap();
                            }

                            self.builder.position_at_end(elif_else);
                        }
                    }

                    if let Some(else_body) = else_branch {
                        for stmt in else_body {
                            self.compile_statement(stmt)?;
                        }
                    }

                    if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                        self.builder.build_unconditional_branch(merge_block).unwrap();
                    }

                    self.builder.position_at_end(merge_block);
                }

                Ok(())
            }

            Statement::While { condition, body } => {
                let function = self
                    .current_function
                    .ok_or("While loop outside of function")?;

                let cond_block = self.context.append_basic_block(function, "while_cond");
                let body_block = self.context.append_basic_block(function, "while_body");
                let after_block = self.context.append_basic_block(function, "after_while");

                self.builder.build_unconditional_branch(cond_block).unwrap();

                self.builder.position_at_end(cond_block);
                let cond_value = self.compile_expression(condition)?;
                let cond_bool = cond_value.into_int_value();
                self.builder
                    .build_conditional_branch(cond_bool, body_block, after_block)
                    .unwrap();

                self.builder.position_at_end(body_block);

                // Push loop context for break/continue
                self.loop_stack.push(LoopContext {
                    continue_block: cond_block,
                    break_block: after_block,
                });

                for stmt in body {
                    self.compile_statement(stmt)?;
                }

                // Pop loop context
                self.loop_stack.pop();

                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(cond_block).unwrap();
                }

                self.builder.position_at_end(after_block);

                Ok(())
            }

            Statement::For { variable, iterable, body } => {
                // Desugar for loop to while loop:
                // for item in list {
                //     body
                // }
                // =>
                // _idx = 0
                // while _idx < list.length {
                //     item = list[_idx]
                //     body
                //     _idx = _idx + 1
                // }

                let function = self
                    .current_function
                    .ok_or("For loop outside of function")?;

                // Evaluate iterable once and store it
                let iterable_val = self.compile_expression(iterable)?;
                let iterable_type = iterable_val.get_type();
                let iterable_alloca = self.builder.build_alloca(iterable_type, "_iterable").unwrap();
                self.builder.build_store(iterable_alloca, iterable_val).unwrap();

                // Determine if iterating over a string or a list
                let is_string = if let Expression::Variable(var_name) = iterable {
                    if let Some((_ptr, _llvm_type, ast_type)) = self.variables.get(var_name) {
                        ast_type == &Type::Str
                    } else {
                        false
                    }
                } else if matches!(iterable, Expression::StringLiteral(_)) {
                    true
                } else {
                    false
                };

                // Get length using appropriate function
                let iterable_loaded = self.builder.build_load(iterable_type, iterable_alloca, "").unwrap();
                let length_fn = if is_string {
                    self.functions.get("str_length").unwrap()
                } else {
                    self.functions.get("list_length").unwrap()
                };
                let length = self
                    .builder
                    .build_call(*length_fn, &[iterable_loaded.into()], "length")
                    .unwrap()
                    .try_as_basic_value()
                    .left()
                    .unwrap();

                // Create index variable
                let i64_type = self.context.i64_type();
                let idx_alloca = self.builder.build_alloca(i64_type, "_idx").unwrap();
                self.builder.build_store(idx_alloca, i64_type.const_zero()).unwrap();

                // Create blocks for while loop
                let cond_block = self.context.append_basic_block(function, "for_cond");
                let body_block = self.context.append_basic_block(function, "for_body");
                let incr_block = self.context.append_basic_block(function, "for_incr");
                let after_block = self.context.append_basic_block(function, "for_end");

                // Jump to condition
                self.builder.build_unconditional_branch(cond_block).unwrap();

                // Condition block: idx < length
                self.builder.position_at_end(cond_block);
                let idx = self.builder.build_load(i64_type, idx_alloca, "idx").unwrap().into_int_value();
                let length_int = length.into_int_value();
                let cond = self.builder.build_int_compare(
                    inkwell::IntPredicate::SLT,
                    idx,
                    length_int,
                    "cond"
                ).unwrap();
                self.builder.build_conditional_branch(cond, body_block, after_block).unwrap();

                // Body block
                self.builder.position_at_end(body_block);

                // Load item: item = iterable[idx]
                let iterable_loaded = self.builder.build_load(iterable_type, iterable_alloca, "").unwrap();
                let idx_loaded = self.builder.build_load(i64_type, idx_alloca, "").unwrap();

                let (item_val, item_ast_type) = if is_string {
                    // For strings, use str_char_at
                    let str_char_at_fn = self.functions.get("str_char_at").unwrap();
                    let char_val = self
                        .builder
                        .build_call(*str_char_at_fn, &[iterable_loaded.into(), idx_loaded.into()], "char")
                        .unwrap()
                        .try_as_basic_value()
                        .left()
                        .unwrap();
                    (char_val, Type::Str)
                } else {
                    // For lists, use list_get_i64
                    let list_get_fn = self.functions.get("list_get_i64").unwrap();
                    let item_val = self
                        .builder
                        .build_call(*list_get_fn, &[iterable_loaded.into(), idx_loaded.into()], "item")
                        .unwrap()
                        .try_as_basic_value()
                        .left()
                        .unwrap();
                    (item_val, Type::Int)
                };

                // Declare loop variable
                let item_alloca = self.builder.build_alloca(item_val.get_type(), variable).unwrap();
                self.builder.build_store(item_alloca, item_val).unwrap();
                self.variables.insert(variable.clone(), (item_alloca, item_val.get_type(), item_ast_type));

                // Push loop context for break/continue (continue goes to increment block)
                self.loop_stack.push(LoopContext {
                    continue_block: incr_block,
                    break_block: after_block,
                });

                // Compile body statements
                for stmt in body {
                    self.compile_statement(stmt)?;
                }

                // Pop loop context
                self.loop_stack.pop();

                // Jump to increment block if no terminator
                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(incr_block).unwrap();
                }

                // Increment block: idx = idx + 1
                self.builder.position_at_end(incr_block);
                let idx_loaded = self.builder.build_load(i64_type, idx_alloca, "idx").unwrap().into_int_value();
                let one = i64_type.const_int(1, false);
                let next_idx = self.builder.build_int_add(idx_loaded, one, "next_idx").unwrap();
                self.builder.build_store(idx_alloca, next_idx).unwrap();
                self.builder.build_unconditional_branch(cond_block).unwrap();

                // After block
                self.builder.position_at_end(after_block);

                // Remove loop variable from scope
                self.variables.remove(variable);

                Ok(())
            }

            Statement::Return(expr) => {
                if let Some(e) = expr {
                    // Compute return value first (may call other functions)
                    let return_value = self.compile_expression(e)?;

                    // Release all RC variables before returning
                    self.release_scope_variables();

                    // Pop function from call stack after computing return value
                    let pop_call_stack_fn = *self.functions.get("pop_call_stack").unwrap();
                    self.builder.build_call(pop_call_stack_fn, &[], "").unwrap();

                    self.builder.build_return(Some(&return_value)).unwrap();
                } else {
                    // Release all RC variables before returning
                    self.release_scope_variables();

                    // Pop function from call stack before returning
                    let pop_call_stack_fn = *self.functions.get("pop_call_stack").unwrap();
                    self.builder.build_call(pop_call_stack_fn, &[], "").unwrap();

                    self.builder.build_return(None).unwrap();
                }
                Ok(())
            }

            Statement::Break => {
                let loop_context = self.loop_stack.last()
                    .ok_or("Break statement outside of loop")?;
                self.builder.build_unconditional_branch(loop_context.break_block).unwrap();
                Ok(())
            }

            Statement::Continue => {
                let loop_context = self.loop_stack.last()
                    .ok_or("Continue statement outside of loop")?;
                self.builder.build_unconditional_branch(loop_context.continue_block).unwrap();
                Ok(())
            }

            Statement::Assert { condition, message } => {
                let function = self.current_function.ok_or("Assert outside of function")?;

                // Evaluate condition
                let cond_value = self.compile_expression(condition)?;
                let cond_bool = cond_value.into_int_value();

                // Create basic blocks
                let fail_block = self.context.append_basic_block(function, "assert_fail");
                let continue_block = self.context.append_basic_block(function, "assert_continue");

                // Branch based on condition
                self.builder.build_conditional_branch(cond_bool, continue_block, fail_block).unwrap();

                // Fail block: print error and exit
                self.builder.position_at_end(fail_block);

                // Create error message
                let error_msg = if let Some(msg) = message {
                    format!("Assertion failed: {}\n", msg)
                } else {
                    "Assertion failed\n".to_string()
                };
                let error_str = self.builder.build_global_string_ptr(&error_msg, "assert_msg").unwrap();

                // Call printf
                let printf_fn = self.module.get_function("printf").unwrap();
                self.builder.build_call(printf_fn, &[error_str.as_basic_value_enum().into()], "").unwrap();

                // Call exit(1)
                let i32_type = self.context.i32_type();
                let exit_fn = self.module.get_function("exit").unwrap_or_else(|| {
                    let exit_type = self.context.void_type().fn_type(&[i32_type.into()], false);
                    self.module.add_function("exit", exit_type, None)
                });
                self.builder.build_call(exit_fn, &[i32_type.const_int(1, false).into()], "").unwrap();
                self.builder.build_unreachable().unwrap();

                // Continue block: assertion passed
                self.builder.position_at_end(continue_block);
                Ok(())
            }

            Statement::Try { try_block, except_clauses, finally_block } => {
                let function = self.current_function.ok_or("Try statement outside of function")?;

                // Allocate jmp_buf on stack (200 bytes)
                let jmp_buf_type = self.context.i8_type().array_type(200);
                let jmp_buf_alloca = self.builder.build_alloca(jmp_buf_type, "jmp_buf").unwrap();

                // Push exception handler
                let exception_push_handler_fn = *self.functions.get("exception_push_handler").unwrap();
                self.builder.build_call(
                    exception_push_handler_fn,
                    &[jmp_buf_alloca.into()],
                    ""
                ).unwrap();

                // Call setjmp
                let setjmp_fn = *self.functions.get("setjmp").unwrap();
                let setjmp_result = self.builder.build_call(
                    setjmp_fn,
                    &[jmp_buf_alloca.into()],
                    "setjmp_result"
                ).unwrap().try_as_basic_value().left().unwrap().into_int_value();

                // Check if setjmp returned 0 (normal) or 1 (exception)
                let is_normal = self.builder.build_int_compare(
                    IntPredicate::EQ,
                    setjmp_result,
                    self.context.i32_type().const_zero(),
                    "is_normal"
                ).unwrap();

                let try_normal_block = self.context.append_basic_block(function, "try_normal");
                let try_exception_block = self.context.append_basic_block(function, "try_exception");
                let finally_block_label = self.context.append_basic_block(function, "finally");
                let end_block = self.context.append_basic_block(function, "try_end");

                self.builder.build_conditional_branch(is_normal, try_normal_block, try_exception_block).unwrap();

                // Normal path: execute try block
                self.builder.position_at_end(try_normal_block);
                for stmt in try_block {
                    self.compile_statement(stmt)?;
                }
                // If we reach here, no exception was raised
                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(finally_block_label).unwrap();
                }

                // Exception path: match and handle exception
                self.builder.position_at_end(try_exception_block);

                // Get current exception
                let exception_get_current_fn = *self.functions.get("exception_get_current").unwrap();
                let current_exc = self.builder.build_call(
                    exception_get_current_fn,
                    &[],
                    "current_exc"
                ).unwrap().try_as_basic_value().left().unwrap().into_pointer_value();

                // Get exception handler functions (needed in finally block)
                let exception_pop_handler_fn = *self.functions.get("exception_pop_handler").unwrap();

                // If no except clauses, jump straight to unhandled
                if except_clauses.is_empty() {
                    let unhandled_block = self.context.append_basic_block(function, "unhandled");
                    self.builder.build_unconditional_branch(unhandled_block).unwrap();

                    // Unhandled exception: pop handler and re-raise
                    self.builder.position_at_end(unhandled_block);
                    self.builder.build_call(exception_pop_handler_fn, &[], "").unwrap();
                    // Execute finally before re-raising
                    if finally_block.is_some() {
                        self.builder.build_unconditional_branch(finally_block_label).unwrap();
                    } else {
                        // TODO: Re-raise the exception
                        self.builder.build_unreachable().unwrap();
                    }
                } else {
                    // Generate except clause matching
                    let mut next_except_block = self.context.append_basic_block(function, "except_check");
                    self.builder.build_unconditional_branch(next_except_block).unwrap();

                    let unhandled_block = self.context.append_basic_block(function, "unhandled");

                    for (i, except_clause) in except_clauses.iter().enumerate() {
                    self.builder.position_at_end(next_except_block);

                    let except_body_block = self.context.append_basic_block(function, &format!("except_body_{}", i));
                    let next_check = if i < except_clauses.len() - 1 {
                        self.context.append_basic_block(function, &format!("except_check_{}", i + 1))
                    } else {
                        unhandled_block
                    };

                    if let Some(ref exc_type) = except_clause.exception_type {
                        // Check if exception matches this type
                        let exc_type_str = self.builder.build_global_string_ptr(exc_type, "exc_type_check").unwrap();
                        let exception_matches_fn = *self.functions.get("exception_matches").unwrap();
                        let matches = self.builder.build_call(
                            exception_matches_fn,
                            &[current_exc.into(), exc_type_str.as_pointer_value().into()],
                            "matches"
                        ).unwrap().try_as_basic_value().left().unwrap().into_int_value();

                        let matches_bool = self.builder.build_int_compare(
                            IntPredicate::NE,
                            matches,
                            self.context.i32_type().const_zero(),
                            "matches_bool"
                        ).unwrap();

                        self.builder.build_conditional_branch(matches_bool, except_body_block, next_check).unwrap();
                    } else {
                        // Catch-all except clause
                        self.builder.build_unconditional_branch(except_body_block).unwrap();
                    }

                    // Execute except body
                    self.builder.position_at_end(except_body_block);

                    // If there's a variable binding, declare it
                    if let Some(ref var_name) = except_clause.var_name {
                        let exc_ptr_type = self.context.ptr_type(AddressSpace::default());
                        let exc_var_alloca = self.builder.build_alloca(exc_ptr_type, var_name).unwrap();
                        self.builder.build_store(exc_var_alloca, current_exc).unwrap();
                        self.variables.insert(var_name.clone(), (exc_var_alloca, exc_ptr_type.as_basic_type_enum(), Type::Exception));
                    }

                    for stmt in &except_clause.body {
                        self.compile_statement(stmt)?;
                    }

                    // Clear exception
                    let exception_clear_fn = *self.functions.get("exception_clear").unwrap();
                    self.builder.build_call(exception_clear_fn, &[], "").unwrap();

                    // Remove variable binding if present
                    if let Some(ref var_name) = except_clause.var_name {
                        self.variables.remove(var_name);
                    }

                    if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                        self.builder.build_unconditional_branch(finally_block_label).unwrap();
                    }

                        next_except_block = next_check;
                    }

                    // Unhandled exception: pop handler and re-raise
                    self.builder.position_at_end(unhandled_block);
                    self.builder.build_call(exception_pop_handler_fn, &[], "").unwrap();
                    // TODO: Re-raise the exception
                    self.builder.build_unreachable().unwrap();
                }

                // Finally block
                self.builder.position_at_end(finally_block_label);

                // Pop exception handler
                self.builder.build_call(exception_pop_handler_fn, &[], "").unwrap();

                if let Some(finally) = finally_block {
                    for stmt in finally {
                        self.compile_statement(stmt)?;
                    }
                }

                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(end_block).unwrap();
                }

                self.builder.position_at_end(end_block);
                Ok(())
            }

            Statement::Raise { exception_type, message, line } => {
                // Compile the message expression
                let message_value = self.compile_expression(message)?;

                // Get exception type as string
                let type_str = self.builder.build_global_string_ptr(exception_type, "exc_type").unwrap();

                // Get source file name
                let file_str = self.builder.build_global_string_ptr(&self.source_file, "exc_file").unwrap();

                // Create line number constant
                let line_const = self.context.i64_type().const_int(*line as u64, false);

                // Call exception_raise(type, message, file, line)
                let exception_raise_fn = *self.functions.get("exception_raise").unwrap();
                self.builder.build_call(
                    exception_raise_fn,
                    &[
                        type_str.as_pointer_value().into(),
                        message_value.into(),
                        file_str.as_pointer_value().into(),
                        line_const.into(),
                    ],
                    ""
                ).unwrap();

                // exception_raise doesn't return, but we need unreachable to mark this
                self.builder.build_unreachable().unwrap();

                Ok(())
            }

            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
                Ok(())
            }

            Statement::Pass => Ok(()),

            Statement::Import { .. } => {
                // Imports are already processed at load time, skip them
                Ok(())
            }
        }
    }

    fn compile_expression(&mut self, expression: &Expression) -> Result<BasicValueEnum<'ctx>, String> {
        match expression {
            Expression::IntLiteral(n) => {
                Ok(self.context.i64_type().const_int(*n as u64, true).as_basic_value_enum())
            }

            Expression::FloatLiteral(f) => {
                Ok(self.context.f64_type().const_float(*f).as_basic_value_enum())
            }

            Expression::StringLiteral(s) => {
                let string_value = self.builder.build_global_string_ptr(s, "str").unwrap();
                Ok(string_value.as_pointer_value().as_basic_value_enum())
            }

            Expression::BoolLiteral(b) => Ok(self
                .context
                .bool_type()
                .const_int(if *b { 1 } else { 0 }, false)
                .as_basic_value_enum()),

            Expression::NoneLiteral => Ok(self
                .context
                .ptr_type(AddressSpace::default())
                .const_null()
                .as_basic_value_enum()),

            Expression::Variable(name) => {
                let (ptr, var_type, _ast_type) = self
                    .variables
                    .get(name)
                    .ok_or(format!("Undefined variable '{}'", name))?;
                Ok(self.builder.build_load(*var_type, *ptr, name).unwrap())
            }

            Expression::Binary { left, op, right } => {
                let left_val = self.compile_expression(left)?;
                let right_val = self.compile_expression(right)?;

                match op {
                    BinaryOp::Add => {
                        // Check for string concatenation first
                        if left_val.is_pointer_value() && right_val.is_pointer_value() {
                            // String concatenation
                            let left_str = left_val.into_pointer_value();
                            let right_str = right_val.into_pointer_value();

                            // Get string lengths
                            let strlen_fn = *self.functions.get("strlen").unwrap();
                            let left_len = self.builder
                                .build_call(strlen_fn, &[left_str.into()], "left_len")
                                .unwrap()
                                .try_as_basic_value()
                                .left()
                                .unwrap()
                                .into_int_value();

                            let right_len = self.builder
                                .build_call(strlen_fn, &[right_str.into()], "right_len")
                                .unwrap()
                                .try_as_basic_value()
                                .left()
                                .unwrap()
                                .into_int_value();

                            // Calculate total length (left_len + right_len + 1 for null terminator)
                            let total_len = self.builder
                                .build_int_add(left_len, right_len, "total_len")
                                .unwrap();
                            let total_len_with_null = self.builder
                                .build_int_add(
                                    total_len,
                                    self.context.i64_type().const_int(1, false),
                                    "total_len_with_null",
                                )
                                .unwrap();

                            // Allocate memory for new string
                            let malloc_fn = *self.functions.get("malloc").unwrap();
                            let new_str = self.builder
                                .build_call(malloc_fn, &[total_len_with_null.into()], "concat_str")
                                .unwrap()
                                .try_as_basic_value()
                                .left()
                                .unwrap()
                                .into_pointer_value();

                            // Copy first string
                            let strcpy_fn = *self.functions.get("strcpy").unwrap();
                            self.builder
                                .build_call(strcpy_fn, &[new_str.into(), left_str.into()], "")
                                .unwrap();

                            // Concatenate second string
                            let strcat_fn = *self.functions.get("strcat").unwrap();
                            self.builder
                                .build_call(strcat_fn, &[new_str.into(), right_str.into()], "")
                                .unwrap();

                            Ok(new_str.as_basic_value_enum())
                        } else if left_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_add(
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "addtmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        } else {
                            Ok(self
                                .builder
                                .build_float_add(
                                    left_val.into_float_value(),
                                    right_val.into_float_value(),
                                    "addtmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        }
                    }

                    BinaryOp::Subtract => {
                        if left_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_sub(
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "subtmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        } else {
                            Ok(self
                                .builder
                                .build_float_sub(
                                    left_val.into_float_value(),
                                    right_val.into_float_value(),
                                    "subtmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        }
                    }

                    BinaryOp::Multiply => {
                        if left_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_mul(
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "multmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        } else {
                            Ok(self
                                .builder
                                .build_float_mul(
                                    left_val.into_float_value(),
                                    right_val.into_float_value(),
                                    "multmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        }
                    }

                    BinaryOp::Divide => {
                        if left_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_signed_div(
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "divtmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        } else {
                            Ok(self
                                .builder
                                .build_float_div(
                                    left_val.into_float_value(),
                                    right_val.into_float_value(),
                                    "divtmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        }
                    }

                    BinaryOp::Modulo => Ok(self
                        .builder
                        .build_int_signed_rem(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "modtmp",
                        )
                        .unwrap()
                        .as_basic_value_enum()),

                    BinaryOp::FloorDivide => Ok(self
                        .builder
                        .build_int_signed_div(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "floordivtmp",
                        )
                        .unwrap()
                        .as_basic_value_enum()),

                    BinaryOp::Power => {
                        Err("Power operator not yet implemented".to_string())
                    }

                    BinaryOp::Equal => {
                        if left_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::EQ,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "eqtmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        } else {
                            Ok(self
                                .builder
                                .build_float_compare(
                                    FloatPredicate::OEQ,
                                    left_val.into_float_value(),
                                    right_val.into_float_value(),
                                    "eqtmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        }
                    }

                    BinaryOp::NotEqual => {
                        if left_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::NE,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "netmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        } else {
                            Ok(self
                                .builder
                                .build_float_compare(
                                    FloatPredicate::ONE,
                                    left_val.into_float_value(),
                                    right_val.into_float_value(),
                                    "netmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        }
                    }

                    BinaryOp::Less => {
                        if left_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::SLT,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "lttmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        } else {
                            Ok(self
                                .builder
                                .build_float_compare(
                                    FloatPredicate::OLT,
                                    left_val.into_float_value(),
                                    right_val.into_float_value(),
                                    "lttmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        }
                    }

                    BinaryOp::Greater => {
                        if left_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::SGT,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "gttmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        } else {
                            Ok(self
                                .builder
                                .build_float_compare(
                                    FloatPredicate::OGT,
                                    left_val.into_float_value(),
                                    right_val.into_float_value(),
                                    "gttmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        }
                    }

                    BinaryOp::LessEqual => {
                        if left_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::SLE,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "letmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        } else {
                            Ok(self
                                .builder
                                .build_float_compare(
                                    FloatPredicate::OLE,
                                    left_val.into_float_value(),
                                    right_val.into_float_value(),
                                    "letmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        }
                    }

                    BinaryOp::GreaterEqual => {
                        if left_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::SGE,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "getmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        } else {
                            Ok(self
                                .builder
                                .build_float_compare(
                                    FloatPredicate::OGE,
                                    left_val.into_float_value(),
                                    right_val.into_float_value(),
                                    "getmp",
                                )
                                .unwrap()
                                .as_basic_value_enum())
                        }
                    }

                    BinaryOp::And => Ok(self
                        .builder
                        .build_and(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "andtmp",
                        )
                        .unwrap()
                        .as_basic_value_enum()),

                    BinaryOp::Or => Ok(self
                        .builder
                        .build_or(
                            left_val.into_int_value(),
                            right_val.into_int_value(),
                            "ortmp",
                        )
                        .unwrap()
                        .as_basic_value_enum()),
                }
            }

            Expression::Unary { op, operand } => {
                let operand_val = self.compile_expression(operand)?;

                match op {
                    UnaryOp::Not => Ok(self
                        .builder
                        .build_not(operand_val.into_int_value(), "nottmp")
                        .unwrap()
                        .as_basic_value_enum()),

                    UnaryOp::Negate => {
                        if operand_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_neg(operand_val.into_int_value(), "negtmp")
                                .unwrap()
                                .as_basic_value_enum())
                        } else {
                            Ok(self
                                .builder
                                .build_float_neg(operand_val.into_float_value(), "negtmp")
                                .unwrap()
                                .as_basic_value_enum())
                        }
                    }
                }
            }

            Expression::Call { callee, args, line } => {
                // Set debug location for this call
                let scope = if let Some(func_scope) = self.current_debug_scope {
                    func_scope.as_debug_info_scope()
                } else {
                    self.compile_unit.get_file().as_debug_info_scope()
                };
                let debug_loc = self.debug_builder.create_debug_location(
                    self.context,
                    *line as u32,
                    0, // column
                    scope,
                    None,
                );
                self.builder.set_current_debug_location(debug_loc);

                // Check if this is a module.function() call
                if let Expression::MemberAccess { object, member } = &**callee {
                    if let Expression::Variable(_module_name) = &**object {
                        // Module.function() call - extract function name and call it
                        // Module name already validated by type checker, just use function name
                        let function = if let Some(&func) = self.functions.get(member) {
                            func
                        } else if let Some(func) = self.module.get_function(member) {
                            func
                        } else {
                            return Err(format!("Undefined function '{}'", member));
                        };

                        let mut arg_values: Vec<BasicMetadataValueEnum> = Vec::new();
                        for arg in args {
                            let arg_val = self.compile_expression(arg)?;
                            arg_values.push(arg_val.into());
                        }

                        let call_site_value = self
                            .builder
                            .build_call(function, &arg_values, "calltmp")
                            .unwrap();

                        if let Some(return_value) = call_site_value.try_as_basic_value().left() {
                            return Ok(return_value);
                        } else {
                            return Ok(self.context.i64_type().const_zero().as_basic_value_enum());
                        }
                    }
                }

                if let Expression::Variable(func_name) = &**callee {
                    // Handle range() as a special built-in
                    if func_name == "range" {
                        if args.len() != 1 {
                            return Err("range() takes exactly 1 argument".to_string());
                        }

                        let n = self.compile_expression(&args[0])?;
                        let n_int = n.into_int_value();

                        // Create empty list
                        let list_create = *self.functions.get("list_create_i64").unwrap();
                        let list_ptr = self
                            .builder
                            .build_call(list_create, &[], "range_list")
                            .unwrap()
                            .try_as_basic_value()
                            .left()
                            .unwrap()
                            .into_pointer_value();

                        let function = self.current_function.ok_or("range() outside of function")?;

                        // Create loop blocks
                        let loop_header = self.context.append_basic_block(function, "range_loop_header");
                        let loop_body = self.context.append_basic_block(function, "range_loop_body");
                        let loop_exit = self.context.append_basic_block(function, "range_loop_exit");

                        // Create counter variable
                        let i64_type = self.context.i64_type();
                        let counter = self.builder.build_alloca(i64_type, "range_counter").unwrap();
                        self.builder.build_store(counter, i64_type.const_zero()).unwrap();

                        // Jump to loop header
                        self.builder.build_unconditional_branch(loop_header).unwrap();

                        // Loop header: check i < n
                        self.builder.position_at_end(loop_header);
                        let i_val = self.builder.build_load(i64_type, counter, "i").unwrap().into_int_value();
                        let cond = self.builder.build_int_compare(
                            inkwell::IntPredicate::SLT,
                            i_val,
                            n_int,
                            "range_cond"
                        ).unwrap();
                        self.builder.build_conditional_branch(cond, loop_body, loop_exit).unwrap();

                        // Loop body: push i to list, increment i
                        self.builder.position_at_end(loop_body);
                        let i_val = self.builder.build_load(i64_type, counter, "i").unwrap();
                        let list_push = *self.functions.get("list_push_i64").unwrap();
                        self.builder.build_call(
                            list_push,
                            &[list_ptr.into(), i_val.into()],
                            ""
                        ).unwrap();

                        let next_i = self.builder.build_int_add(
                            i_val.into_int_value(),
                            i64_type.const_int(1, false),
                            "next_i"
                        ).unwrap();
                        self.builder.build_store(counter, next_i).unwrap();
                        self.builder.build_unconditional_branch(loop_header).unwrap();

                        // After loop
                        self.builder.position_at_end(loop_exit);
                        return Ok(list_ptr.as_basic_value_enum());
                    }

                    let function = if let Some(&func) = self.functions.get(func_name) {
                        func
                    } else if let Some(func) = self.module.get_function(func_name) {
                        func
                    } else {
                        return Err(format!("Undefined function '{}'", func_name));
                    };

                    let mut arg_values: Vec<BasicMetadataValueEnum> = Vec::new();
                    for arg in args {
                        let arg_val = self.compile_expression(arg)?;
                        arg_values.push(arg_val.into());
                    }

                    let call_site_value = self
                        .builder
                        .build_call(function, &arg_values, "calltmp")
                        .unwrap();

                    if let Some(return_value) = call_site_value.try_as_basic_value().left() {
                        Ok(return_value)
                    } else {
                        Ok(self.context.i64_type().const_zero().as_basic_value_enum())
                    }
                } else {
                    Err("Only simple function calls are supported".to_string())
                }
            }

            Expression::MemberAccess { object, member } => {
                // Check if this is a field access on a class instance
                if let Expression::Variable(var_name) = &**object {
                    if let Some((_ptr, _llvm_type, ast_type)) = self.variables.get(var_name) {
                        if let Type::Custom(class_name) = ast_type {
                            // This is a class instance field access
                            let struct_type = *self.class_types.get(class_name).unwrap();
                            let field_names = self.class_fields.get(class_name).unwrap().clone();

                            // Find field index
                            if let Some(field_idx) = field_names.iter().position(|f| f == member) {
                                // Get the object pointer
                                let obj_val = self.compile_expression(object)?;
                                let obj_ptr = obj_val.into_pointer_value();

                                // Get field type from struct
                                let field_type = struct_type.get_field_type_at_index(field_idx as u32).unwrap();

                                // Get field pointer
                                let field_ptr = self
                                    .builder
                                    .build_struct_gep(struct_type, obj_ptr, field_idx as u32, member)
                                    .unwrap();

                                // Load the field value
                                let field_val = self
                                    .builder
                                    .build_load(field_type, field_ptr, member)
                                    .unwrap();

                                return Ok(field_val);
                            }
                        }
                    }
                }

                // Handle .length property for lists and strings
                if member == "length" {
                    let obj_val = self.compile_expression(object)?;

                    // Determine the type of object to call the right function
                    // Try to get the type from the variable if it's a variable reference
                    let use_str_length = if let Expression::Variable(var_name) = &**object {
                        if let Some((_ptr, _llvm_type, ast_type)) = self.variables.get(var_name) {
                            ast_type == &Type::Str
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    let length_fn = if use_str_length {
                        self.functions.get("str_length").unwrap()
                    } else {
                        self.functions.get("list_length").unwrap()
                    };

                    let length = self
                        .builder
                        .build_call(*length_fn, &[obj_val.into()], "length")
                        .unwrap()
                        .try_as_basic_value()
                        .left()
                        .unwrap();
                    Ok(length)
                } else {
                    Err(format!("Member access '{}' not implemented", member))
                }
            }

            Expression::Assignment { target, value } => {
                let var_info = self.variables.get(target)
                    .ok_or(format!("Undefined variable '{}'", target))?;
                let ptr = var_info.0;
                let var_type = var_info.1;
                let ast_type = var_info.2.clone();

                let new_val = self.compile_expression(value)?;

                // Add RC logic for ref-counted types
                if self.is_rc_type(&ast_type) && new_val.is_pointer_value() {
                    let new_ptr = new_val.into_pointer_value();

                    // Retain new value
                    self.build_rc_retain_inline(new_ptr);

                    // Load and release old value
                    let old_val = self.builder.build_load(var_type, ptr, "old_val").unwrap();
                    if old_val.is_pointer_value() {
                        let old_ptr = old_val.into_pointer_value();
                        // Check if not null before releasing
                        let is_null = self.builder.build_is_null(old_ptr, "is_null").unwrap();
                        let function = self.current_function.unwrap();
                        let release_block = self.context.append_basic_block(function, "release_old");
                        let store_block = self.context.append_basic_block(function, "store_new");

                        self.builder.build_conditional_branch(is_null, store_block, release_block).unwrap();

                        self.builder.position_at_end(release_block);
                        self.build_rc_release_inline(old_ptr);
                        self.builder.build_unconditional_branch(store_block).unwrap();

                        self.builder.position_at_end(store_block);
                    }
                }

                self.builder.build_store(ptr, new_val).unwrap();
                Ok(new_val)
            }

            Expression::ArrayLiteral { .. } => {
                Err("Array literals not yet fully implemented in codegen".to_string())
            }

            Expression::ListLiteral { elements } => {
                // For now, only support int lists
                // Create empty list
                let list_create = self.functions.get("list_create_i64").unwrap();
                let list_ptr = self
                    .builder
                    .build_call(*list_create, &[], "list")
                    .unwrap()
                    .try_as_basic_value()
                    .left()
                    .unwrap();

                // Add each element by calling list_push_i64
                if !elements.is_empty() {
                    let list_push = *self.functions.get("list_push_i64").unwrap();

                    for element in elements {
                        let element_value = self.compile_expression(element)?;
                        self.builder
                            .build_call(list_push, &[list_ptr.into(), element_value.into()], "")
                            .unwrap();
                    }
                }

                Ok(list_ptr)
            }

            Expression::DictLiteral { pairs } => {
                // Create empty dict
                let dict_create = self.functions.get("dict_create").unwrap();
                let dict_ptr = self
                    .builder
                    .build_call(*dict_create, &[], "dict")
                    .unwrap()
                    .try_as_basic_value()
                    .left()
                    .unwrap();

                // Add each key-value pair
                if !pairs.is_empty() {
                    let dict_set = *self.functions.get("dict_set").unwrap();

                    for (key_expr, val_expr) in pairs {
                        let key_value = self.compile_expression(key_expr)?;
                        let val_value = self.compile_expression(val_expr)?;

                        // For now, assume keys are strings and values are ints
                        self.builder
                            .build_call(dict_set, &[dict_ptr.into(), key_value.into(), val_value.into()], "")
                            .unwrap();
                    }
                }

                Ok(dict_ptr)
            }

            Expression::Index { object, index, line } => {
                let obj_val = self.compile_expression(object)?;
                let idx_val = self.compile_expression(index)?;

                // Set debug location for this operation
                let scope = if let Some(func_scope) = self.current_debug_scope {
                    func_scope.as_debug_info_scope()
                } else {
                    self.compile_unit.get_file().as_debug_info_scope()
                };
                let debug_loc = self.debug_builder.create_debug_location(
                    self.context,
                    *line as u32,
                    0, // column
                    scope,
                    None,
                );
                self.builder.set_current_debug_location(debug_loc);

                // Check if this is dict access (string key) or list access (int index)
                if idx_val.is_pointer_value() {
                    // Dict access with string key (no line parameter needed)
                    let dict_get = self.functions.get("dict_get").unwrap();
                    let result = self
                        .builder
                        .build_call(*dict_get, &[obj_val.into(), idx_val.into()], "dict_value")
                        .unwrap()
                        .try_as_basic_value()
                        .left()
                        .unwrap();
                    Ok(result)
                } else {
                    // List access with int index (no line parameter needed)
                    let list_get = self.functions.get("list_get_i64").unwrap();
                    let result = self
                        .builder
                        .build_call(*list_get, &[obj_val.into(), idx_val.into()], "element")
                        .unwrap()
                        .try_as_basic_value()
                        .left()
                        .unwrap();
                    Ok(result)
                }
            }

            Expression::IndexAssignment { object, index, value, line } => {
                // Get the object (dict or list) and load its value
                let (obj_ptr, obj_llvm_type, _) = self.variables.get(object)
                    .ok_or_else(|| format!("Undefined variable '{}'", object))?
                    .clone();

                // Load the actual dict/list pointer from the variable
                let obj_val = self.builder.build_load(obj_llvm_type, obj_ptr, object)
                    .unwrap();

                let idx_val = self.compile_expression(index)?;
                let val_val = self.compile_expression(value)?;

                // Set debug location for this operation
                let scope = if let Some(func_scope) = self.current_debug_scope {
                    func_scope.as_debug_info_scope()
                } else {
                    self.compile_unit.get_file().as_debug_info_scope()
                };
                let debug_loc = self.debug_builder.create_debug_location(
                    self.context,
                    *line as u32,
                    0, // column
                    scope,
                    None,
                );
                self.builder.set_current_debug_location(debug_loc);

                // Check if this is dict assignment (string key) or list assignment (int index)
                if idx_val.is_pointer_value() {
                    // Dict assignment with string key
                    let dict_set = self.functions.get("dict_set")
                        .ok_or("dict_set function not found")?;
                    self.builder.build_call(*dict_set,
                        &[obj_val.into(), idx_val.into(), val_val.into()], "")
                        .unwrap();
                } else {
                    // List assignment with int index (no line parameter needed)
                    let list_set = self.functions.get("list_set_i64")
                        .ok_or("list_set_i64 function not found")?;
                    self.builder.build_call(*list_set,
                        &[obj_val.into(), idx_val.into(), val_val.into()], "")
                        .unwrap();
                }

                // Return void
                Ok(self.context.i64_type().const_zero().as_basic_value_enum())
            }

            Expression::MethodCall { object, method, args } => {
                // Check if this is a class method call FIRST
                if let Expression::Variable(var_name) = &**object {
                    if let Some((_ptr, _llvm_type, ast_type)) = self.variables.get(var_name) {
                        if let Type::Custom(class_name) = ast_type {
                            // This is a class method call
                            let method_full_name = format!("{}::{}", class_name, method);
                            if let Some(&func) = self.functions.get(&method_full_name) {
                                // Get the object value (pointer to struct)
                                let obj_val = self.compile_expression(object)?;

                                // Build arguments: self + user args
                                let mut arg_values: Vec<BasicMetadataValueEnum> = vec![obj_val.into()];
                                for arg in args {
                                    let arg_val = self.compile_expression(arg)?;
                                    arg_values.push(arg_val.into());
                                }

                                let call_site_value = self
                                    .builder
                                    .build_call(func, &arg_values, "method_call")
                                    .unwrap();

                                if let Some(return_value) = call_site_value.try_as_basic_value().left() {
                                    return Ok(return_value);
                                } else {
                                    return Ok(self.context.i64_type().const_zero().as_basic_value_enum());
                                }
                            }
                        }
                    }

                    // If not a class instance, check if this is a module.function() call
                    // Check if this method exists as a regular function
                    if let Some(&func) = self.functions.get(method) {
                        // This is a module function call
                        let mut arg_values: Vec<BasicMetadataValueEnum> = Vec::new();
                        for arg in args {
                            let arg_val = self.compile_expression(arg)?;
                            arg_values.push(arg_val.into());
                        }

                        let call_site_value = self
                            .builder
                            .build_call(func, &arg_values, "calltmp")
                            .unwrap();

                        if let Some(return_value) = call_site_value.try_as_basic_value().left() {
                            return Ok(return_value);
                        } else {
                            return Ok(self.context.i64_type().const_zero().as_basic_value_enum());
                        }
                    } else if let Some(func) = self.module.get_function(method) {
                        // This is a module function call from the current module
                        let mut arg_values: Vec<BasicMetadataValueEnum> = Vec::new();
                        for arg in args {
                            let arg_val = self.compile_expression(arg)?;
                            arg_values.push(arg_val.into());
                        }

                        let call_site_value = self
                            .builder
                            .build_call(func, &arg_values, "calltmp")
                            .unwrap();

                        if let Some(return_value) = call_site_value.try_as_basic_value().left() {
                            return Ok(return_value);
                        } else {
                            return Ok(self.context.i64_type().const_zero().as_basic_value_enum());
                        }
                    }
                }

                let obj_val = self.compile_expression(object)?;

                match method.as_str() {
                    "push" => {
                        if args.len() != 1 {
                            return Err("push() takes exactly 1 argument".to_string());
                        }
                        let arg_val = self.compile_expression(&args[0])?;
                        let list_push = *self.functions.get("list_push_i64").unwrap();
                        self.builder
                            .build_call(list_push, &[obj_val.into(), arg_val.into()], "")
                            .unwrap();
                        // push returns void, return a dummy value
                        Ok(self.context.i64_type().const_zero().as_basic_value_enum())
                    }

                    "pop" => {
                        if !args.is_empty() {
                            return Err("pop() takes no arguments".to_string());
                        }
                        let list_pop = *self.functions.get("list_pop_i64").unwrap();
                        let result = self
                            .builder
                            .build_call(list_pop, &[obj_val.into()], "pop_result")
                            .unwrap()
                            .try_as_basic_value()
                            .left()
                            .unwrap();
                        Ok(result)
                    }

                    "get" => {
                        if args.len() != 1 {
                            return Err("get() takes exactly 1 argument".to_string());
                        }
                        let idx_val = self.compile_expression(&args[0])?;
                        let list_get = *self.functions.get("list_get_i64").unwrap();
                        let result = self
                            .builder
                            .build_call(list_get, &[obj_val.into(), idx_val.into()], "get_result")
                            .unwrap()
                            .try_as_basic_value()
                            .left()
                            .unwrap();
                        Ok(result)
                    }

                    "upper" => {
                        if !args.is_empty() {
                            return Err("upper() takes no arguments".to_string());
                        }
                        let str_upper = *self.functions.get("str_upper").unwrap();
                        let result = self
                            .builder
                            .build_call(str_upper, &[obj_val.into()], "upper_result")
                            .unwrap()
                            .try_as_basic_value()
                            .left()
                            .unwrap();
                        Ok(result)
                    }

                    "lower" => {
                        if !args.is_empty() {
                            return Err("lower() takes no arguments".to_string());
                        }
                        let str_lower = *self.functions.get("str_lower").unwrap();
                        let result = self
                            .builder
                            .build_call(str_lower, &[obj_val.into()], "lower_result")
                            .unwrap()
                            .try_as_basic_value()
                            .left()
                            .unwrap();
                        Ok(result)
                    }

                    "contains" => {
                        if args.len() != 1 {
                            return Err("contains() takes exactly 1 argument".to_string());
                        }
                        let arg_val = self.compile_expression(&args[0])?;
                        let str_contains = *self.functions.get("str_contains").unwrap();
                        let result = self
                            .builder
                            .build_call(str_contains, &[obj_val.into(), arg_val.into()], "contains_result")
                            .unwrap()
                            .try_as_basic_value()
                            .left()
                            .unwrap();
                        // Convert i32 result to i64 for consistency
                        let result_i64 = self.builder.build_int_z_extend(
                            result.into_int_value(),
                            self.context.i64_type(),
                            "contains_i64"
                        ).unwrap();
                        Ok(result_i64.as_basic_value_enum())
                    }

                    _ => Err(format!("Unknown method '{}'", method)),
                }
            }

            Expression::FString { parts, expressions } => {
                // F-string implementation: concatenate parts and formatted expressions
                let i64_type = self.context.i64_type();

                // Start with a reasonably sized buffer to avoid buffer overflow
                // Using 1024 bytes which should be enough for most f-strings
                let malloc_fn = *self.functions.get("malloc").unwrap();
                let initial_size = i64_type.const_int(1024, false);
                let result_str = self.builder
                    .build_call(malloc_fn, &[initial_size.into()], "fstring_result")
                    .unwrap()
                    .try_as_basic_value()
                    .left()
                    .unwrap()
                    .into_pointer_value();

                // Initialize with empty string (null terminator at start)
                self.builder.build_store(result_str, i64_type.const_int(0, false)).unwrap();

                let strcat_fn = *self.functions.get("strcat").unwrap();
                let sprintf_fn = *self.functions.get("sprintf").unwrap();

                // Iterate through parts and expressions
                for (i, part) in parts.iter().enumerate() {
                    // Add the string part if not empty
                    if !part.is_empty() {
                        let part_str = self.builder.build_global_string_ptr(part, &format!("fstr_part_{}", i)).unwrap();
                        self.builder.build_call(strcat_fn, &[result_str.into(), part_str.as_pointer_value().into()], "").unwrap();
                    }

                    // Add the expression value if there is one
                    if i < expressions.len() {
                        let expr_val = self.compile_expression(&expressions[i])?;

                        // Allocate buffer for formatted value (100 bytes should be enough)
                        let buffer_size = i64_type.const_int(100, false);
                        let buffer = self.builder
                            .build_call(malloc_fn, &[buffer_size.into()], &format!("expr_buffer_{}", i))
                            .unwrap()
                            .try_as_basic_value()
                            .left()
                            .unwrap()
                            .into_pointer_value();

                        // Format the value based on its type
                        if expr_val.is_int_value() {
                            let fmt = self.builder.build_global_string_ptr("%lld", "int_fmt").unwrap();
                            self.builder.build_call(
                                sprintf_fn,
                                &[buffer.into(), fmt.as_pointer_value().into(), expr_val.into()],
                                ""
                            ).unwrap();
                        } else if expr_val.is_float_value() {
                            let fmt = self.builder.build_global_string_ptr("%g", "float_fmt").unwrap();
                            self.builder.build_call(
                                sprintf_fn,
                                &[buffer.into(), fmt.as_pointer_value().into(), expr_val.into()],
                                ""
                            ).unwrap();
                        } else if expr_val.is_pointer_value() {
                            // Assume it's a string
                            let fmt = self.builder.build_global_string_ptr("%s", "str_fmt").unwrap();
                            self.builder.build_call(
                                sprintf_fn,
                                &[buffer.into(), fmt.as_pointer_value().into(), expr_val.into()],
                                ""
                            ).unwrap();
                        }

                        // Concatenate the formatted value
                        self.builder.build_call(strcat_fn, &[result_str.into(), buffer.into()], "").unwrap();
                    }
                }

                Ok(result_str.as_basic_value_enum())
            }
        }
    }

    fn generate_constructor(&mut self, class_name: &str, fields: &[Field]) -> Result<(), String> {
        // Get the struct type
        let struct_type = *self.class_types.get(class_name).unwrap();
        let ptr_type = self.context.ptr_type(AddressSpace::default());

        // Create constructor function signature
        let param_types: Vec<BasicMetadataTypeEnum> = fields
            .iter()
            .map(|f| self.get_llvm_type(&f.field_type).into())
            .collect();

        let fn_type = ptr_type.fn_type(&param_types, false);
        let function = self.module.add_function(class_name, fn_type, None);
        self.functions.insert(class_name.to_string(), function);

        // Create entry block
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // Allocate memory for the struct
        let size = struct_type.size_of().unwrap();
        let malloc_fn = self.functions.get("malloc").unwrap();
        let ptr = self
            .builder
            .build_call(*malloc_fn, &[size.into()], "obj_ptr")
            .unwrap()
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        // Initialize each field
        for (i, _field) in fields.iter().enumerate() {
            let field_ptr = self
                .builder
                .build_struct_gep(struct_type, ptr, i as u32, &format!("field_{}", i))
                .unwrap();
            let param_val = function.get_nth_param(i as u32).unwrap();
            self.builder.build_store(field_ptr, param_val).unwrap();
        }

        // Call init method if it exists
        let init_method_name = format!("{}::init", class_name);
        if let Some(&init_fn) = self.functions.get(&init_method_name) {
            self.builder
                .build_call(init_fn, &[ptr.into()], "init_call")
                .unwrap();
        }

        // Return the pointer
        self.builder.build_return(Some(&ptr)).unwrap();

        Ok(())
    }
}
