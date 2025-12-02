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
                    _ => {
                        return Err(format!(
                            "Cannot iterate over type {}. Only list, array, and dict are iterable.",
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

            Expression::Call { callee, args } => {
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

                // Handle .length property for arrays and lists
                if member == "length" {
                    match obj_type {
                        Type::Array(_, _) | Type::List(_) => Ok(Type::Int),
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

            Expression::Index { object, index } => {
                let obj_type = self.check_expression(object)?;
                let idx_type = self.check_expression(index)?;

                match obj_type {
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

            Expression::IndexAssignment { object, index, value } => {
                let obj_type = self.lookup_variable(object)
                    .ok_or_else(|| format!("Undefined variable '{}'", object))?;
                let idx_type = self.check_expression(index)?;
                let val_type = self.check_expression(value)?;

                match &obj_type {
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

    fn types_compatible(&self, expected: &Type, actual: &Type) -> bool {
        match (expected, actual) {
            (Type::Float, Type::Int) => true,
            (Type::Array(e1, s1), Type::Array(e2, s2)) => {
                s1 == s2 && self.types_compatible(e1, e2)
            }
            (Type::List(e1), Type::List(e2)) => self.types_compatible(e1, e2),
            (Type::Dict(k1, v1), Type::Dict(k2, v2)) => {
                self.types_compatible(k1, k2) && self.types_compatible(v1, v2)
            }
            _ => expected == actual,
        }
    }
}
