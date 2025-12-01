use crate::ast::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, StructType};
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, PointerValue};
use inkwell::{AddressSpace, IntPredicate, FloatPredicate};
use std::collections::HashMap;

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
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

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
            Type::Custom(_) => self
                .context
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum(),
        }
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<(), String> {
        self.declare_printf();
        self.declare_memory_functions();
        self.declare_builtin_functions();
        self.declare_list_functions();
        self.declare_dict_functions();

        for statement in &program.statements {
            self.compile_statement(statement)?;
        }

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
    }

    fn declare_builtin_functions(&mut self) {
        let i32_type = self.context.i32_type();
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

        // Allocate list struct (24 bytes)
        let malloc = self.module.get_function("malloc").unwrap();
        let struct_size = self.context.i64_type().const_int(24, false);
        let list_ptr = self.builder
            .build_call(malloc, &[struct_size.into()], "list_ptr")
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

        // dict_get(dict_ptr, key_str) -> i64 (returns 0 if not found)
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
                    self.builder.build_store(alloca, init_value).unwrap();
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

                let entry = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(entry);

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
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
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

                // Get length
                let iterable_loaded = self.builder.build_load(iterable_type, iterable_alloca, "").unwrap();
                let list_length_fn = self.functions.get("list_length").unwrap();
                let length = self
                    .builder
                    .build_call(*list_length_fn, &[iterable_loaded.into()], "length")
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

                // Load item: item = list[idx]
                let iterable_loaded = self.builder.build_load(iterable_type, iterable_alloca, "").unwrap();
                let idx_loaded = self.builder.build_load(i64_type, idx_alloca, "").unwrap();
                let list_get_fn = self.functions.get("list_get_i64").unwrap();
                let item_val = self
                    .builder
                    .build_call(*list_get_fn, &[iterable_loaded.into(), idx_loaded.into()], "item")
                    .unwrap()
                    .try_as_basic_value()
                    .left()
                    .unwrap();

                // Declare loop variable
                let item_alloca = self.builder.build_alloca(item_val.get_type(), variable).unwrap();
                self.builder.build_store(item_alloca, item_val).unwrap();
                // TODO: Infer proper element type from iterable
                self.variables.insert(variable.clone(), (item_alloca, item_val.get_type(), Type::Int));

                // Compile body statements
                for stmt in body {
                    self.compile_statement(stmt)?;
                }

                // Increment: idx = idx + 1
                let idx_loaded = self.builder.build_load(i64_type, idx_alloca, "idx").unwrap().into_int_value();
                let one = i64_type.const_int(1, false);
                let next_idx = self.builder.build_int_add(idx_loaded, one, "next_idx").unwrap();
                self.builder.build_store(idx_alloca, next_idx).unwrap();

                // Jump back to condition
                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(cond_block).unwrap();
                }

                // After block
                self.builder.position_at_end(after_block);

                // Remove loop variable from scope
                self.variables.remove(variable);

                Ok(())
            }

            Statement::Return(expr) => {
                if let Some(e) = expr {
                    let return_value = self.compile_expression(e)?;
                    self.builder.build_return(Some(&return_value)).unwrap();
                } else {
                    self.builder.build_return(None).unwrap();
                }
                Ok(())
            }

            Statement::Break | Statement::Continue => {
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

            Expression::Call { callee, args } => {
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

                // Handle .length property for lists
                if member == "length" {
                    let obj_val = self.compile_expression(object)?;
                    let list_length = self.functions.get("list_length").unwrap();
                    let length = self
                        .builder
                        .build_call(*list_length, &[obj_val.into()], "length")
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
                let (ptr, _, _) = *self
                    .variables
                    .get(target)
                    .ok_or(format!("Undefined variable '{}'", target))?;
                let val = self.compile_expression(value)?;
                self.builder.build_store(ptr, val).unwrap();
                Ok(val)
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

            Expression::Index { object, index } => {
                let obj_val = self.compile_expression(object)?;
                let idx_val = self.compile_expression(index)?;

                // Check if this is dict access (string key) or list access (int index)
                if idx_val.is_pointer_value() {
                    // Dict access with string key
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
                    // List access with int index
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

            Expression::IndexAssignment { object, index, value } => {
                // Get the object (dict or list) and load its value
                let (obj_ptr, obj_llvm_type, _) = self.variables.get(object)
                    .ok_or_else(|| format!("Undefined variable '{}'", object))?
                    .clone();

                // Load the actual dict/list pointer from the variable
                let obj_val = self.builder.build_load(obj_llvm_type, obj_ptr, object)
                    .unwrap();

                let idx_val = self.compile_expression(index)?;
                let val_val = self.compile_expression(value)?;

                // Check if this is dict assignment (string key) or list assignment (int index)
                if idx_val.is_pointer_value() {
                    // Dict assignment with string key
                    let dict_set = self.functions.get("dict_set")
                        .ok_or("dict_set function not found")?;
                    self.builder.build_call(*dict_set,
                        &[obj_val.into(), idx_val.into(), val_val.into()], "")
                        .unwrap();
                } else {
                    // List assignment with int index
                    let list_set = self.functions.get("list_set")
                        .ok_or("list_set function not found")?;
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

                    _ => Err(format!("Unknown method '{}' on list", method)),
                }
            }

            Expression::FString { parts, expressions } => {
                // F-string implementation: concatenate parts and formatted expressions
                let ptr_type = self.context.ptr_type(AddressSpace::default());
                let i64_type = self.context.i64_type();

                // Start with empty result string
                let malloc_fn = *self.functions.get("malloc").unwrap();
                let initial_size = i64_type.const_int(1, false);
                let result_str = self.builder
                    .build_call(malloc_fn, &[initial_size.into()], "fstring_result")
                    .unwrap()
                    .try_as_basic_value()
                    .left()
                    .unwrap()
                    .into_pointer_value();

                // Initialize with empty string
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
