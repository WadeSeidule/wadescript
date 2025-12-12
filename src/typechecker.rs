use crate::ast::*;
use std::collections::HashMap;

struct ClassInfo {
    fields: Vec<(String, Type)>, // Ordered fields for constructor
    field_map: HashMap<String, Type>, // Quick lookup for field access
}

pub struct TypeChecker {
    symbol_table: Vec<HashMap<String, Type>>,
    functions: HashMap<String, (Vec<Type>, Type)>,
    classes: HashMap<String, ClassInfo>,
    current_function_return_type: Option<Type>,
    modules: HashMap<String, Vec<String>>, // module_name -> function_names
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut functions = HashMap::new();

        // Register built-in print functions
        functions.insert("print_int".to_string(), (vec![Type::Int], Type::Void));
        functions.insert("print_float".to_string(), (vec![Type::Float], Type::Void));
        functions.insert("print_str".to_string(), (vec![Type::Str], Type::Void));
        functions.insert("print_bool".to_string(), (vec![Type::Bool], Type::Void));

        // Register built-in utility functions
        functions.insert("range".to_string(), (vec![Type::Int], Type::List(Box::new(Type::Int))));

        // Register file I/O functions (used by std/io.ws)
        functions.insert("file_open".to_string(), (vec![Type::Str, Type::Str], Type::Int));
        functions.insert("file_read".to_string(), (vec![Type::Int], Type::Str));
        functions.insert("file_read_line".to_string(), (vec![Type::Int], Type::Str));
        functions.insert("file_write".to_string(), (vec![Type::Int, Type::Str], Type::Void));
        functions.insert("file_close".to_string(), (vec![Type::Int], Type::Void));
        functions.insert("file_exists".to_string(), (vec![Type::Str], Type::Int));

        // Register CLI functions (used by std/cli.ws)
        functions.insert("cli_get_argc".to_string(), (vec![], Type::Int));
        functions.insert("cli_get_argv".to_string(), (vec![Type::Int], Type::Str));
        functions.insert("cli_get_argv_copy".to_string(), (vec![Type::Int], Type::Str));
        functions.insert("cli_parse_int".to_string(), (vec![Type::Str], Type::Int));
        functions.insert("cli_parse_bool".to_string(), (vec![Type::Str], Type::Int));
        functions.insert("cli_starts_with".to_string(), (vec![Type::Str, Type::Str], Type::Int));
        functions.insert("cli_str_eq".to_string(), (vec![Type::Str, Type::Str], Type::Int));
        functions.insert("cli_after_prefix".to_string(), (vec![Type::Str, Type::Str], Type::Str));

        // Register HTTP functions (used by std/http.ws)
        functions.insert("http_get".to_string(), (vec![Type::Str], Type::Int));
        functions.insert("http_get_with_headers".to_string(), (vec![Type::Str, Type::Str], Type::Int));
        functions.insert("http_post".to_string(), (vec![Type::Str, Type::Str, Type::Str], Type::Int));
        functions.insert("http_put".to_string(), (vec![Type::Str, Type::Str, Type::Str], Type::Int));
        functions.insert("http_delete".to_string(), (vec![Type::Str, Type::Str], Type::Int));
        functions.insert("http_patch".to_string(), (vec![Type::Str, Type::Str, Type::Str], Type::Int));
        functions.insert("http_head".to_string(), (vec![Type::Str, Type::Str], Type::Int));
        functions.insert("http_response_status".to_string(), (vec![Type::Int], Type::Int));
        functions.insert("http_response_body".to_string(), (vec![Type::Int], Type::Str));
        functions.insert("http_response_headers".to_string(), (vec![Type::Int], Type::Str));
        functions.insert("http_response_get_header".to_string(), (vec![Type::Int, Type::Str], Type::Str));
        functions.insert("http_response_free".to_string(), (vec![Type::Int], Type::Void));

        TypeChecker {
            symbol_table: vec![HashMap::new()],
            functions,
            classes: HashMap::new(),
            current_function_return_type: None,
            modules: HashMap::new(),
        }
    }

    fn enter_scope(&mut self) {
        self.symbol_table.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.symbol_table.pop();
    }

    fn declare_variable(&mut self, name: String, var_type: Type) {
        if let Some(scope) = self.symbol_table.last_mut() {
            scope.insert(name, var_type);
        }
    }

    fn lookup_variable(&self, name: &str) -> Option<Type> {
        for scope in self.symbol_table.iter().rev() {
            if let Some(var_type) = scope.get(name) {
                return Some(var_type.clone());
            }
        }
        None
    }

    /// Register a REPL variable in the global scope (for variable persistence)
    pub fn register_repl_variable(&mut self, name: &str, var_type: &Type) {
        if let Some(scope) = self.symbol_table.first_mut() {
            scope.insert(name.to_string(), var_type.clone());
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), String> {
        // Store module information
        self.modules = program.modules.clone();

        for statement in &program.statements {
            self.check_statement(statement)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::VarDecl {
                name,
                type_annotation,
                initializer,
            } => {
                if let Some(init_expr) = initializer {
                    // For empty list literals, use the type annotation
                    let init_type = if let Expression::ListLiteral { elements } = init_expr {
                        if elements.is_empty() {
                            type_annotation.clone()
                        } else {
                            self.check_expression(init_expr)?
                        }
                    } else if let Expression::DictLiteral { pairs } = init_expr {
                        // For empty dict literals, use the type annotation
                        if pairs.is_empty() {
                            type_annotation.clone()
                        } else {
                            self.check_expression(init_expr)?
                        }
                    } else {
                        self.check_expression(init_expr)?
                    };

                    if !self.types_compatible(type_annotation, &init_type) {
                        return Err(format!(
                            "Type mismatch in variable '{}': expected {}, got {}",
                            name, type_annotation, init_type
                        ));
                    }
                }
                self.declare_variable(name.clone(), type_annotation.clone());
                Ok(())
            }

            Statement::FunctionDef {
                name,
                params,
                return_type,
                body,
            } => {
                let param_types: Vec<Type> = params.iter().map(|p| p.param_type.clone()).collect();
                self.functions
                    .insert(name.clone(), (param_types, return_type.clone()));

                self.enter_scope();
                self.current_function_return_type = Some(return_type.clone());

                for param in params {
                    self.declare_variable(param.name.clone(), param.param_type.clone());
                }

                for stmt in body {
                    self.check_statement(stmt)?;
                }

                self.current_function_return_type = None;
                self.exit_scope();
                Ok(())
            }

            Statement::ClassDef {
                name,
                _base_class: _,
                fields,
                methods,
            } => {
                // Validate decorators on fields
                for field in fields {
                    self.validate_field_decorators(name, field)?;
                }

                // Store class fields in order and in a map
                let mut ordered_fields = Vec::new();
                let mut field_map = HashMap::new();
                for field in fields {
                    ordered_fields.push((field.name.clone(), field.field_type.clone()));
                    field_map.insert(field.name.clone(), field.field_type.clone());
                }

                let class_info = ClassInfo {
                    fields: ordered_fields,
                    field_map,
                };
                self.classes.insert(name.clone(), class_info);

                // Register methods as functions with Class::method naming
                for method in methods {
                    if let Statement::FunctionDef {
                        name: method_name,
                        params,
                        return_type,
                        body: _,
                    } = method
                    {
                        let param_types: Vec<Type> =
                            params.iter().map(|p| p.param_type.clone()).collect();
                        self.functions.insert(
                            format!("{}::{}", name, method_name),
                            (param_types, return_type.clone()),
                        );
                    }
                }

                // Type check methods
                for method in methods {
                    self.check_statement(method)?;
                }

                Ok(())
            }

            Statement::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => {
                let cond_type = self.check_expression(condition)?;
                if cond_type != Type::Bool {
                    return Err(format!(
                        "If condition must be bool, got {}",
                        cond_type
                    ));
                }

                self.enter_scope();
                for stmt in then_branch {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();

                for (elif_cond, elif_body) in elif_branches {
                    let elif_cond_type = self.check_expression(elif_cond)?;
                    if elif_cond_type != Type::Bool {
                        return Err(format!(
                            "Elif condition must be bool, got {}",
                            elif_cond_type
                        ));
                    }

                    self.enter_scope();
                    for stmt in elif_body {
                        self.check_statement(stmt)?;
                    }
                    self.exit_scope();
                }

                if let Some(else_body) = else_branch {
                    self.enter_scope();
                    for stmt in else_body {
                        self.check_statement(stmt)?;
                    }
                    self.exit_scope();
                }

                Ok(())
            }

            Statement::While { condition, body } => {
                let cond_type = self.check_expression(condition)?;
                if cond_type != Type::Bool {
                    return Err(format!(
                        "While condition must be bool, got {}",
                        cond_type
                    ));
                }

                self.enter_scope();
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();

                Ok(())
            }

            Statement::For {
                variable,
                iterable,
                body,
            } => {
                // Check iterable type and determine element type
                let iterable_type = self.check_expression(iterable)?;

                let element_type = match iterable_type {
                    Type::List(elem_type) => *elem_type,
                    Type::Array(elem_type, _) => *elem_type,
                    Type::Dict(key_type, _) => *key_type, // Iterate over keys
                    Type::Str => Type::Str, // Iterate over characters (as strings)
                    _ => {
                        return Err(format!(
                            "Cannot iterate over type {}. Only list, array, dict, and str are iterable.",
                            iterable_type
                        ));
                    }
                };

                self.enter_scope();
                self.declare_variable(variable.clone(), element_type);

                for stmt in body {
                    self.check_statement(stmt)?;
                }
                self.exit_scope();

                Ok(())
            }

            Statement::Return(expr) => {
                let return_type = if let Some(e) = expr {
                    self.check_expression(e)?
                } else {
                    Type::Void
                };

                if let Some(expected_return_type) = &self.current_function_return_type {
                    if !self.types_compatible(expected_return_type, &return_type) {
                        return Err(format!(
                            "Return type mismatch: expected {}, got {}",
                            expected_return_type, return_type
                        ));
                    }
                }

                Ok(())
            }

            Statement::Assert { condition, .. } => {
                let cond_type = self.check_expression(condition)?;
                if cond_type != Type::Bool {
                    return Err(format!("Assert condition must be bool, got {}", cond_type));
                }
                Ok(())
            }

            Statement::Try { try_block, except_clauses, finally_block } => {
                // Type check try block
                for stmt in try_block {
                    self.check_statement(stmt)?;
                }

                // Type check except clauses
                for except_clause in except_clauses {
                    // If there's a variable binding, declare it with type Exception
                    if let Some(ref var_name) = except_clause.var_name {
                        self.enter_scope();
                        self.declare_variable(var_name.clone(), Type::Exception);
                    }

                    for stmt in &except_clause.body {
                        self.check_statement(stmt)?;
                    }

                    if except_clause.var_name.is_some() {
                        self.exit_scope();
                    }
                }

                // Type check finally block
                if let Some(finally) = finally_block {
                    for stmt in finally {
                        self.check_statement(stmt)?;
                    }
                }

                Ok(())
            }

            Statement::Raise { exception_type: _, message, line: _ } => {
                // Check that message is a string
                let msg_type = self.check_expression(message)?;
                if msg_type != Type::Str {
                    return Err(format!("Exception message must be str, got {}", msg_type));
                }
                Ok(())
            }

            Statement::Break | Statement::Continue | Statement::Pass | Statement::Import { .. } => Ok(()),

            Statement::Expression(expr) => {
                self.check_expression(expr)?;
                Ok(())
            }
        }
    }

    fn check_expression(&mut self, expression: &Expression) -> Result<Type, String> {
        match expression {
            Expression::IntLiteral(_) => Ok(Type::Int),
            Expression::FloatLiteral(_) => Ok(Type::Float),
            Expression::StringLiteral(_) => Ok(Type::Str),
            Expression::BoolLiteral(_) => Ok(Type::Bool),
            Expression::NoneLiteral => Ok(Type::Void),

            Expression::Variable(name) => self
                .lookup_variable(name)
                .ok_or_else(|| format!("Undefined variable '{}'", name)),

            Expression::Binary { left, op, right } => {
                let left_type = self.check_expression(left)?;
                let right_type = self.check_expression(right)?;

                match op {
                    BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide => {
                        if (left_type == Type::Int || left_type == Type::Float)
                            && (right_type == Type::Int || right_type == Type::Float)
                        {
                            if left_type == Type::Float || right_type == Type::Float {
                                Ok(Type::Float)
                            } else {
                                Ok(Type::Int)
                            }
                        } else if left_type == Type::Str
                            && right_type == Type::Str
                            && *op == BinaryOp::Add
                        {
                            Ok(Type::Str)
                        } else {
                            Err(format!(
                                "Invalid operands for {:?}: {} and {}",
                                op, left_type, right_type
                            ))
                        }
                    }

                    BinaryOp::Modulo | BinaryOp::FloorDivide => {
                        if left_type == Type::Int && right_type == Type::Int {
                            Ok(Type::Int)
                        } else {
                            Err(format!(
                                "Invalid operands for {:?}: {} and {}",
                                op, left_type, right_type
                            ))
                        }
                    }

                    BinaryOp::Power => {
                        if (left_type == Type::Int || left_type == Type::Float)
                            && (right_type == Type::Int || right_type == Type::Float)
                        {
                            if left_type == Type::Float || right_type == Type::Float {
                                Ok(Type::Float)
                            } else {
                                Ok(Type::Int)
                            }
                        } else {
                            Err(format!(
                                "Invalid operands for power: {} and {}",
                                left_type, right_type
                            ))
                        }
                    }

                    BinaryOp::Equal
                    | BinaryOp::NotEqual
                    | BinaryOp::Less
                    | BinaryOp::Greater
                    | BinaryOp::LessEqual
                    | BinaryOp::GreaterEqual => {
                        if self.types_compatible(&left_type, &right_type) {
                            Ok(Type::Bool)
                        } else {
                            Err(format!(
                                "Cannot compare {} and {}",
                                left_type, right_type
                            ))
                        }
                    }

                    BinaryOp::And | BinaryOp::Or => {
                        if left_type == Type::Bool && right_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err(format!(
                                "Logical operators require bool operands, got {} and {}",
                                left_type, right_type
                            ))
                        }
                    }
                }
            }

            Expression::Unary { op, operand } => {
                let operand_type = self.check_expression(operand)?;
                match op {
                    UnaryOp::Not => {
                        if operand_type == Type::Bool {
                            Ok(Type::Bool)
                        } else {
                            Err(format!(
                                "Not operator requires bool operand, got {}",
                                operand_type
                            ))
                        }
                    }
                    UnaryOp::Negate => {
                        if operand_type == Type::Int || operand_type == Type::Float {
                            Ok(operand_type)
                        } else {
                            Err(format!(
                                "Negate operator requires numeric operand, got {}",
                                operand_type
                            ))
                        }
                    }
                }
            }

            Expression::Call { callee, args, line: _ } => {
                // Check if this is a module.function() call
                if let Expression::MemberAccess { object, member } = &**callee {
                    if let Expression::Variable(module_name) = &**object {
                        // Check if this is a known module
                        if let Some(module_functions) = self.modules.get(module_name) {
                            // Check if the function exists in this module
                            if !module_functions.contains(member) {
                                return Err(format!(
                                    "Module '{}' has no function '{}'",
                                    module_name, member
                                ));
                            }

                            // Look up the function signature
                            if let Some((param_types, return_type)) = self.functions.get(member).cloned() {
                                if args.len() != param_types.len() {
                                    return Err(format!(
                                        "Function '{}.{}' expects {} arguments, got {}",
                                        module_name,
                                        member,
                                        param_types.len(),
                                        args.len()
                                    ));
                                }

                                for (i, arg) in args.iter().enumerate() {
                                    let arg_type = self.check_expression(arg)?;
                                    if !self.types_compatible(&param_types[i], &arg_type) {
                                        return Err(format!(
                                            "Argument {} of function '{}.{}': expected {}, got {}",
                                            i + 1,
                                            module_name,
                                            member,
                                            param_types[i],
                                            arg_type
                                        ));
                                    }
                                }

                                return Ok(return_type);
                            } else {
                                return Err(format!("Undefined function '{}'", member));
                            }
                        }
                    }
                }

                // Check if this is a class constructor call
                if let Expression::Variable(class_name) = &**callee {
                    if let Some(class_info) = self.classes.get(class_name) {
                        // This is a constructor call - arguments must match field types in order
                        let field_types: Vec<Type> = class_info.fields.iter()
                            .map(|(_, field_type)| field_type.clone())
                            .collect();

                        if args.len() != field_types.len() {
                            return Err(format!(
                                "Constructor for '{}' expects {} arguments, got {}",
                                class_name,
                                field_types.len(),
                                args.len()
                            ));
                        }

                        for (i, arg) in args.iter().enumerate() {
                            let arg_type = self.check_expression(arg)?;
                            if !self.types_compatible(&field_types[i], &arg_type) {
                                return Err(format!(
                                    "Argument {} of constructor '{}': expected {}, got {}",
                                    i + 1,
                                    class_name,
                                    field_types[i],
                                    arg_type
                                ));
                            }
                        }

                        return Ok(Type::Custom(class_name.clone()));
                    }
                }

                // Regular function call
                if let Expression::Variable(func_name) = &**callee {
                    if let Some((param_types, return_type)) = self.functions.get(func_name).cloned() {
                        if args.len() != param_types.len() {
                            return Err(format!(
                                "Function '{}' expects {} arguments, got {}",
                                func_name,
                                param_types.len(),
                                args.len()
                            ));
                        }

                        for (i, arg) in args.iter().enumerate() {
                            let arg_type = self.check_expression(arg)?;
                            if !self.types_compatible(&param_types[i], &arg_type) {
                                return Err(format!(
                                    "Argument {} of function '{}': expected {}, got {}",
                                    i + 1,
                                    func_name,
                                    param_types[i],
                                    arg_type
                                ));
                            }
                        }

                        Ok(return_type)
                    } else {
                        Err(format!("Undefined function '{}'", func_name))
                    }
                } else {
                    Err("Only simple function calls are supported".to_string())
                }
            }

            Expression::MemberAccess { object, member } => {
                // Check if this is a module.function reference
                if let Expression::Variable(module_name) = &**object {
                    if self.modules.contains_key(module_name) {
                        // This is a module reference - it will be validated in the Call/MethodCall context
                        // For now, return void as a placeholder since this should only appear in calls
                        return Ok(Type::Void);
                    }
                }

                let obj_type = self.check_expression(object)?;

                // Handle field access on custom types (classes)
                if let Type::Custom(class_name) = &obj_type {
                    if let Some(class_info) = self.classes.get(class_name) {
                        // Check if field exists
                        if let Some(field_type) = class_info.field_map.get(member) {
                            // Check for private access
                            if member.starts_with('_') {
                                return Err(format!(
                                    "Cannot access private field '{}' of class '{}'",
                                    member, class_name
                                ));
                            }
                            return Ok(field_type.clone());
                        } else {
                            return Err(format!(
                                "Class '{}' has no field '{}'",
                                class_name, member
                            ));
                        }
                    }
                }

                // Handle .length property for arrays, lists, and strings
                // Also handle Optional types by unwrapping and checking inner type
                if member == "length" {
                    match &obj_type {
                        Type::Array(_, _) | Type::List(_) | Type::Str => Ok(Type::Int),
                        Type::Optional(inner) => {
                            // Allow .length on Optional if inner type supports it
                            match inner.as_ref() {
                                Type::Array(_, _) | Type::List(_) | Type::Str => Ok(Type::Int),
                                _ => Err(format!("Type {} has no property '{}'", obj_type, member)),
                            }
                        }
                        _ => Err(format!("Type {} has no property '{}'", obj_type, member)),
                    }
                } else {
                    Err(format!("Unknown property '{}' on type {}", member, obj_type))
                }
            }

            Expression::Assignment { target, value } => {
                let var_type = self
                    .lookup_variable(target)
                    .ok_or_else(|| format!("Undefined variable '{}'", target))?;
                let value_type = self.check_expression(value)?;

                if !self.types_compatible(&var_type, &value_type) {
                    return Err(format!(
                        "Cannot assign {} to variable '{}' of type {}",
                        value_type, target, var_type
                    ));
                }

                Ok(var_type)
            }

            Expression::ListLiteral { elements } => {
                if elements.is_empty() {
                    return Err("Cannot infer type of empty list literal".to_string());
                }

                let first_type = self.check_expression(&elements[0])?;
                for elem in &elements[1..] {
                    let elem_type = self.check_expression(elem)?;
                    if !self.types_compatible(&first_type, &elem_type) {
                        return Err(format!(
                            "Inconsistent types in list literal: expected {}, got {}",
                            first_type, elem_type
                        ));
                    }
                }

                Ok(Type::List(Box::new(first_type)))
            }

            Expression::ArrayLiteral { elements } => {
                if elements.is_empty() {
                    return Err("Cannot infer type of empty array literal".to_string());
                }

                let first_type = self.check_expression(&elements[0])?;
                for elem in &elements[1..] {
                    let elem_type = self.check_expression(elem)?;
                    if !self.types_compatible(&first_type, &elem_type) {
                        return Err(format!(
                            "Inconsistent types in array literal: expected {}, got {}",
                            first_type, elem_type
                        ));
                    }
                }

                Ok(Type::Array(Box::new(first_type), elements.len()))
            }

            Expression::DictLiteral { pairs } => {
                if pairs.is_empty() {
                    return Err("Cannot infer type of empty dict literal".to_string());
                }

                let (first_key, first_val) = &pairs[0];
                let key_type = self.check_expression(first_key)?;
                let val_type = self.check_expression(first_val)?;

                for (k, v) in &pairs[1..] {
                    let k_type = self.check_expression(k)?;
                    let v_type = self.check_expression(v)?;

                    if !self.types_compatible(&key_type, &k_type) {
                        return Err(format!(
                            "Inconsistent key types in dict: expected {}, got {}",
                            key_type, k_type
                        ));
                    }
                    if !self.types_compatible(&val_type, &v_type) {
                        return Err(format!(
                            "Inconsistent value types in dict: expected {}, got {}",
                            val_type, v_type
                        ));
                    }
                }

                Ok(Type::Dict(Box::new(key_type), Box::new(val_type)))
            }

            Expression::Index { object, index, line: _ } => {
                let obj_type = self.check_expression(object)?;
                let idx_type = self.check_expression(index)?;

                // Handle Optional types by unwrapping
                let base_type = match &obj_type {
                    Type::Optional(inner) => inner.as_ref().clone(),
                    other => other.clone(),
                };

                match base_type {
                    Type::Array(elem_type, _) | Type::List(elem_type) => {
                        if idx_type != Type::Int {
                            return Err(format!(
                                "Array/List index must be int, got {}",
                                idx_type
                            ));
                        }
                        Ok(*elem_type)
                    }
                    Type::Dict(key_type, val_type) => {
                        if !self.types_compatible(&key_type, &idx_type) {
                            return Err(format!(
                                "Dict key type mismatch: expected {}, got {}",
                                key_type, idx_type
                            ));
                        }
                        Ok(*val_type)
                    }
                    _ => Err(format!("Cannot index into type {}", obj_type)),
                }
            }

            Expression::IndexAssignment { object, index, value, line: _ } => {
                let obj_type = self.lookup_variable(object)
                    .ok_or_else(|| format!("Undefined variable '{}'", object))?;
                let idx_type = self.check_expression(index)?;
                let val_type = self.check_expression(value)?;

                // Handle Optional types by unwrapping
                let base_type = match &obj_type {
                    Type::Optional(inner) => inner.as_ref(),
                    other => other,
                };

                match base_type {
                    Type::Array(elem_type, _) | Type::List(elem_type) => {
                        if idx_type != Type::Int {
                            return Err(format!("Array/List index must be int, got {}", idx_type));
                        }
                        if !self.types_compatible(elem_type, &val_type) {
                            return Err(format!(
                                "Cannot assign {} to {}[int] (expected {})",
                                val_type, obj_type, elem_type
                            ));
                        }
                        Ok(Type::Void)
                    }
                    Type::Dict(key_type, elem_type) => {
                        if !self.types_compatible(key_type, &idx_type) {
                            return Err(format!(
                                "Dict key type mismatch: expected {}, got {}",
                                key_type, idx_type
                            ));
                        }
                        if !self.types_compatible(elem_type, &val_type) {
                            return Err(format!(
                                "Cannot assign {} to {}[{}] (expected {})",
                                val_type, obj_type, key_type, elem_type
                            ));
                        }
                        Ok(Type::Void)
                    }
                    _ => Err(format!("Cannot index assign into type {}", obj_type)),
                }
            }

            Expression::MethodCall { object, method, args } => {
                // Check if this is a module.function() call
                if let Expression::Variable(module_name) = &**object {
                    if let Some(module_functions) = self.modules.get(module_name) {
                        // This is a module function call
                        if !module_functions.contains(method) {
                            return Err(format!(
                                "Module '{}' has no function '{}'",
                                module_name, method
                            ));
                        }

                        // Look up the function signature
                        if let Some((param_types, return_type)) = self.functions.get(method).cloned() {
                            if args.len() != param_types.len() {
                                return Err(format!(
                                    "Function '{}.{}' expects {} arguments, got {}",
                                    module_name,
                                    method,
                                    param_types.len(),
                                    args.len()
                                ));
                            }

                            for (i, arg) in args.iter().enumerate() {
                                let arg_type = self.check_expression(arg)?;
                                if !self.types_compatible(&param_types[i], &arg_type) {
                                    return Err(format!(
                                        "Argument {} of function '{}.{}': expected {}, got {}",
                                        i + 1,
                                        module_name,
                                        method,
                                        param_types[i],
                                        arg_type
                                    ));
                                }
                            }

                            return Ok(return_type);
                        } else {
                            return Err(format!("Undefined function '{}'", method));
                        }
                    }
                }

                let obj_type = self.check_expression(object)?;

                // Handle class methods
                if let Type::Custom(class_name) = &obj_type {
                    // Check for private method access
                    if method.starts_with('_') {
                        return Err(format!(
                            "Cannot access private method '{}' of class '{}'",
                            method, class_name
                        ));
                    }

                    // Look up the method in functions as Class::method
                    let method_full_name = format!("{}::{}", class_name, method);
                    if let Some((param_types, return_type)) = self.functions.get(&method_full_name).cloned() {
                        // First parameter should be self
                        if param_types.is_empty() {
                            return Err(format!(
                                "Method '{}' of class '{}' must have 'self' parameter",
                                method, class_name
                            ));
                        }

                        // Check arguments (skip first param which is self)
                        let method_params = &param_types[1..];
                        if args.len() != method_params.len() {
                            return Err(format!(
                                "Method '{}.{}' expects {} arguments, got {}",
                                class_name,
                                method,
                                method_params.len(),
                                args.len()
                            ));
                        }

                        for (i, arg) in args.iter().enumerate() {
                            let arg_type = self.check_expression(arg)?;
                            if !self.types_compatible(&method_params[i], &arg_type) {
                                return Err(format!(
                                    "Argument {} of method '{}.{}': expected {}, got {}",
                                    i + 1,
                                    class_name,
                                    method,
                                    method_params[i],
                                    arg_type
                                ));
                            }
                        }

                        return Ok(return_type);
                    } else {
                        return Err(format!(
                            "Class '{}' has no method '{}'",
                            class_name, method
                        ));
                    }
                }

                match obj_type {
                    Type::List(elem_type) => match method.as_str() {
                        "push" => {
                            if args.len() != 1 {
                                return Err("push() takes exactly 1 argument".to_string());
                            }
                            let arg_type = self.check_expression(&args[0])?;
                            if !self.types_compatible(&elem_type, &arg_type) {
                                return Err(format!(
                                    "push() argument type mismatch: expected {}, got {}",
                                    elem_type, arg_type
                                ));
                            }
                            Ok(Type::Void)
                        }
                        "pop" => {
                            if !args.is_empty() {
                                return Err("pop() takes no arguments".to_string());
                            }
                            Ok(*elem_type)
                        }
                        "get" => {
                            if args.len() != 1 {
                                return Err("get() takes exactly 1 argument".to_string());
                            }
                            let idx_type = self.check_expression(&args[0])?;
                            if idx_type != Type::Int {
                                return Err("get() index must be int".to_string());
                            }
                            Ok(*elem_type)
                        }
                        _ => Err(format!("Unknown method '{}' on list", method)),
                    },
                    Type::Str => match method.as_str() {
                        "upper" | "lower" => {
                            if !args.is_empty() {
                                return Err(format!("{}() takes no arguments", method));
                            }
                            Ok(Type::Str)
                        }
                        "contains" => {
                            if args.len() != 1 {
                                return Err("contains() takes exactly 1 argument".to_string());
                            }
                            let arg_type = self.check_expression(&args[0])?;
                            if arg_type != Type::Str {
                                return Err(format!(
                                    "contains() argument must be str, got {}",
                                    arg_type
                                ));
                            }
                            Ok(Type::Bool)
                        }
                        "split" => {
                            if args.len() != 1 {
                                return Err("split() takes exactly 1 argument".to_string());
                            }
                            let arg_type = self.check_expression(&args[0])?;
                            if arg_type != Type::Str {
                                return Err(format!(
                                    "split() argument must be str, got {}",
                                    arg_type
                                ));
                            }
                            Ok(Type::List(Box::new(Type::Str)))
                        }
                        _ => Err(format!("Unknown method '{}' on str", method)),
                    },
                    _ => Err(format!("Type {} has no methods", obj_type)),
                }
            }

            Expression::FString { parts: _, expressions } => {
                // Type check all embedded expressions
                for expr in expressions {
                    self.check_expression(expr)?;
                }
                // F-strings always result in a string
                Ok(Type::Str)
            }
        }
    }

    /// Validate decorators on a class field
    fn validate_field_decorators(&self, class_name: &str, field: &Field) -> Result<(), String> {
        for decorator in &field.decorators {
            match decorator.name.as_str() {
                "arg" => {
                    // @arg decorator is for positional arguments, only valid on str fields
                    if field.field_type != Type::Str {
                        return Err(format!(
                            "Class '{}': @arg decorator on field '{}' requires type str, got {}",
                            class_name, field.name, field.field_type
                        ));
                    }
                }
                "option" => {
                    // @option decorator is for named options, valid on str, int, bool
                    match &field.field_type {
                        Type::Str | Type::Int | Type::Bool => {}
                        _ => {
                            return Err(format!(
                                "Class '{}': @option decorator on field '{}' requires type str, int, or bool, got {}",
                                class_name, field.name, field.field_type
                            ));
                        }
                    }

                    // Validate 'short' argument if present - must be single character
                    if let Some(short_val) = decorator.args.get("short") {
                        if short_val.len() != 1 {
                            return Err(format!(
                                "Class '{}': @option decorator on field '{}' has invalid short='{}', must be single character",
                                class_name, field.name, short_val
                            ));
                        }
                    }
                }
                other => {
                    return Err(format!(
                        "Class '{}': Unknown decorator '@{}' on field '{}'",
                        class_name, other, field.name
                    ));
                }
            }
        }
        Ok(())
    }

    fn types_compatible(&self, expected: &Type, actual: &Type) -> bool {
        match (expected, actual) {
            // Float accepts Int
            (Type::Float, Type::Int) => true,
            // Array compatibility
            (Type::Array(e1, s1), Type::Array(e2, s2)) => {
                s1 == s2 && self.types_compatible(e1, e2)
            }
            // List compatibility
            (Type::List(e1), Type::List(e2)) => self.types_compatible(e1, e2),
            // Dict compatibility
            (Type::Dict(k1, v1), Type::Dict(k2, v2)) => {
                self.types_compatible(k1, k2) && self.types_compatible(v1, v2)
            }
            // Optional type compatibility:
            // - None (Void) can be assigned to any Optional[T]
            (Type::Optional(_), Type::Void) => true,
            // - T can be assigned to Optional[T]
            (Type::Optional(inner), actual) => self.types_compatible(inner, actual),
            // - Optional[T] == Optional[T] if inner types match
            // (handled by default case since Type derives PartialEq)
            _ => expected == actual,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn typecheck_source(source: &str) -> Result<(), String> {
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        let mut typechecker = TypeChecker::new();
        typechecker.check_program(&program)
    }

    #[test]
    fn test_variable_declaration_int() {
        assert!(typecheck_source("x: int = 42").is_ok());
    }

    #[test]
    fn test_variable_declaration_float() {
        assert!(typecheck_source("x: float = 3.14").is_ok());
    }

    #[test]
    fn test_variable_declaration_bool() {
        assert!(typecheck_source("x: bool = True").is_ok());
    }

    #[test]
    fn test_variable_declaration_str() {
        assert!(typecheck_source(r#"x: str = "hello""#).is_ok());
    }

    #[test]
    fn test_type_mismatch_int_str() {
        let result = typecheck_source(r#"x: int = "hello""#);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type mismatch"));
    }

    #[test]
    fn test_float_accepts_int() {
        // Float should accept Int (type compatibility)
        assert!(typecheck_source("x: float = 42").is_ok());
    }

    #[test]
    fn test_undefined_variable() {
        let result = typecheck_source("y = x");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_function_definition() {
        let source = r#"
def add(a: int, b: int) -> int {
    return a + b
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_function_call() {
        let source = r#"
def add(a: int, b: int) -> int {
    return a + b
}
def main() -> int {
    x: int = add(5, 10)
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_function_call_wrong_arg_count() {
        let source = r#"
def add(a: int, b: int) -> int {
    return a + b
}
def main() -> int {
    x: int = add(5)
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 2 arguments"));
    }

    #[test]
    fn test_function_call_wrong_arg_type() {
        let source = r#"
def add(a: int, b: int) -> int {
    return a + b
}
def main() -> int {
    x: int = add(5, "hello")
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Argument 2"));
    }

    #[test]
    fn test_return_type_mismatch() {
        let source = r#"
def get_number() -> int {
    return "hello"
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Return type mismatch"));
    }

    #[test]
    fn test_binary_arithmetic() {
        let source = r#"
def main() -> int {
    a: int = 10 + 5
    b: int = 10 - 5
    c: int = 10 * 5
    d: int = 10 / 5
    e: int = 10 % 5
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_binary_arithmetic_float() {
        let source = r#"
def main() -> int {
    a: float = 10.5 + 5.5
    b: float = 10.0 * 2.0
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_binary_arithmetic_mixed() {
        // Int + Float should result in Float
        let source = r#"
def main() -> int {
    a: float = 10 + 5.5
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_comparison_operators() {
        let source = r#"
def main() -> int {
    a: bool = 10 == 5
    b: bool = 10 != 5
    c: bool = 10 < 5
    d: bool = 10 > 5
    e: bool = 10 <= 5
    f: bool = 10 >= 5
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_logical_operators() {
        let source = r#"
def main() -> int {
    a: bool = True and False
    b: bool = True or False
    c: bool = not True
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_logical_operators_wrong_type() {
        let source = r#"
def main() -> int {
    a: bool = 10 and 5
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("bool operands"));
    }

    #[test]
    fn test_if_condition_must_be_bool() {
        let source = r#"
def main() -> int {
    if 10 {
        return 1
    }
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("If condition must be bool"));
    }

    #[test]
    fn test_while_condition_must_be_bool() {
        let source = r#"
def main() -> int {
    while 10 {
        break
    }
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("While condition must be bool"));
    }

    #[test]
    fn test_assert_condition_must_be_bool() {
        let source = r#"
def main() -> int {
    assert 10
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Assert condition must be bool"));
    }

    #[test]
    fn test_list_literal() {
        let source = r#"
def main() -> int {
    nums: list[int] = [1, 2, 3, 4, 5]
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_list_literal_inconsistent_types() {
        let source = r#"
def main() -> int {
    nums: list[int] = [1, 2, "hello"]
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Inconsistent types"));
    }

    #[test]
    fn test_list_index_access() {
        let source = r#"
def main() -> int {
    nums: list[int] = [1, 2, 3]
    x: int = nums[0]
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_list_index_assignment() {
        let source = r#"
def main() -> int {
    nums: list[int] = [1, 2, 3]
    nums[0] = 42
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_list_push_method() {
        let source = r#"
def main() -> int {
    nums: list[int] = [1, 2, 3]
    nums.push(4)
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_list_push_wrong_type() {
        let source = r#"
def main() -> int {
    nums: list[int] = [1, 2, 3]
    nums.push("hello")
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("type mismatch"));
    }

    #[test]
    fn test_list_length_property() {
        let source = r#"
def main() -> int {
    nums: list[int] = [1, 2, 3]
    len: int = nums.length
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_dict_literal() {
        let source = r#"
def main() -> int {
    ages: dict[str, int] = {"Alice": 25, "Bob": 30}
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_dict_inconsistent_key_types() {
        let source = r#"
def main() -> int {
    ages: dict[str, int] = {"Alice": 25, 42: 30}
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Inconsistent key types"));
    }

    #[test]
    fn test_dict_index_access() {
        let source = r#"
def main() -> int {
    ages: dict[str, int] = {"Alice": 25}
    x: int = ages["Alice"]
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_dict_index_assignment() {
        let source = r#"
def main() -> int {
    ages: dict[str, int] = {}
    ages["Alice"] = 25
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_for_loop_over_list() {
        let source = r#"
def main() -> int {
    nums: list[int] = [1, 2, 3]
    for n in nums {
        x: int = n
    }
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_for_loop_over_non_iterable() {
        let source = r#"
def main() -> int {
    for n in 42 {
        pass
    }
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot iterate"));
    }

    #[test]
    fn test_class_definition() {
        let source = r#"
class Person {
    name: str
    age: int

    def greet(self: Person) -> void {
        pass
    }
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_class_constructor() {
        let source = r#"
class Person {
    name: str
    age: int
}
def main() -> int {
    p: Person = Person("Alice", 25)
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_class_constructor_wrong_args() {
        let source = r#"
class Person {
    name: str
    age: int
}
def main() -> int {
    p: Person = Person("Alice")
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 2 arguments"));
    }

    #[test]
    fn test_class_field_access() {
        let source = r#"
class Person {
    name: str
    age: int
}
def main() -> int {
    p: Person = Person("Alice", 25)
    n: str = p.name
    a: int = p.age
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_class_method_call() {
        let source = r#"
class Person {
    name: str

    def greet(self: Person) -> void {
        pass
    }
}
def main() -> int {
    p: Person = Person("Alice")
    p.greet()
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_builtin_functions() {
        let source = r#"
def main() -> int {
    print_int(42)
    print_float(3.14)
    print_str("hello")
    print_bool(True)
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_range_function() {
        let source = r#"
def main() -> int {
    nums: list[int] = range(10)
    for i in range(5) {
        print_int(i)
    }
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_fstring() {
        let source = r#"
def main() -> int {
    name: str = "Alice"
    age: int = 25
    msg: str = f"Name: {name}, Age: {age}"
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_unary_not() {
        let source = r#"
def main() -> int {
    a: bool = not True
    b: bool = not False
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_unary_negate() {
        let source = r#"
def main() -> int {
    a: int = -5
    b: float = -3.14
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_power_operator() {
        let source = r#"
def main() -> int {
    a: int = 2 ** 3
    b: float = 2.0 ** 3.0
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_string_concatenation() {
        let source = r#"
def main() -> int {
    s: str = "Hello" + " " + "World"
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_scope_visibility() {
        let source = r#"
def main() -> int {
    x: int = 10
    if True {
        y: int = 20
        z: int = x + y
    }
    return 0
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_scope_variable_not_visible() {
        let source = r#"
def main() -> int {
    if True {
        x: int = 10
    }
    y: int = x
    return 0
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_decorator_arg_valid() {
        let source = r#"
class Args {
    @arg(help="Input file")
    input_file: str
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_decorator_arg_wrong_type() {
        let source = r#"
class Args {
    @arg(help="Count")
    count: int
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("@arg decorator") && err.contains("requires type str"));
    }

    #[test]
    fn test_decorator_option_valid_types() {
        let source = r#"
class Args {
    @option(short="o", long="output")
    output: str

    @option(short="n", long="number")
    number: int

    @option(short="v", long="verbose")
    verbose: bool
}
"#;
        assert!(typecheck_source(source).is_ok());
    }

    #[test]
    fn test_decorator_option_wrong_type() {
        let source = r#"
class Args {
    @option(long="data")
    data: list[int]
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("@option decorator"));
    }

    #[test]
    fn test_decorator_option_short_single_char() {
        let source = r#"
class Args {
    @option(short="ab", long="output")
    output: str
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("single character"));
    }

    #[test]
    fn test_decorator_unknown() {
        let source = r#"
class Args {
    @unknown(foo="bar")
    field: str
}
"#;
        let result = typecheck_source(source);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown decorator"));
    }
}
