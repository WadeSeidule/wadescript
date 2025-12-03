use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Void,
    Array(Box<Type>, usize),        // Fixed-size array: int[5]
    List(Box<Type>),                // Dynamic list: list[int]
    Dict(Box<Type>, Box<Type>),     // Dictionary: dict[str, int]
    Custom(String),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::Str => write!(f, "str"),
            Type::Void => write!(f, "void"),
            Type::Array(elem_type, size) => write!(f, "{}[{}]", elem_type, size),
            Type::List(elem_type) => write!(f, "list[{}]", elem_type),
            Type::Dict(key_type, val_type) => write!(f, "dict[{}, {}]", key_type, val_type),
            Type::Custom(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub modules: std::collections::HashMap<String, Vec<String>>, // module_name -> function_names
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
            modules: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl {
        name: String,
        type_annotation: Type,
        initializer: Option<Expression>,
    },
    FunctionDef {
        name: String,
        params: Vec<Parameter>,
        return_type: Type,
        body: Vec<Statement>,
    },
    ClassDef {
        name: String,
        _base_class: Option<String>,  // Reserved for future inheritance support
        fields: Vec<Field>,
        methods: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        elif_branches: Vec<(Expression, Vec<Statement>)>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        variable: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    Return(Option<Expression>),
    Break,
    Continue,
    Assert {
        condition: Expression,
        message: Option<String>,
    },
    Expression(Expression),
    Pass,
    Import {
        path: String,
    },
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: Type,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]  // Some variants reserved for future features
pub enum Expression {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    NoneLiteral,
    Variable(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        args: Vec<Expression>,
        line: usize,
    },
    MemberAccess {
        object: Box<Expression>,
        member: String,
    },
    Assignment {
        target: String,
        value: Box<Expression>,
    },
    ArrayLiteral {
        elements: Vec<Expression>,
    },
    ListLiteral {
        elements: Vec<Expression>,
    },
    DictLiteral {
        pairs: Vec<(Expression, Expression)>,
    },
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
        line: usize,
    },
    IndexAssignment {
        object: String,
        index: Box<Expression>,
        value: Box<Expression>,
        line: usize,
    },
    MethodCall {
        object: Box<Expression>,
        method: String,
        args: Vec<Expression>,
    },
    FString {
        parts: Vec<String>,       // String parts between {}
        expressions: Vec<Expression>, // Expressions to interpolate
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    FloorDivide,
    Power,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Negate,
}
