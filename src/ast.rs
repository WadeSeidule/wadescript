use std::collections::HashMap;
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
    Optional(Box<Type>),            // Nullable type: str? or Optional[str]
    Exception,                      // Exception object type
    Tuple(Vec<Type>),               // Tuple type: (int, str, bool)
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
            Type::Optional(inner_type) => write!(f, "{}?", inner_type),
            Type::Exception => write!(f, "Exception"),
            Type::Tuple(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
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
    Try {
        try_block: Vec<Statement>,
        except_clauses: Vec<ExceptClause>,
        finally_block: Option<Vec<Statement>>,
    },
    Raise {
        exception_type: String,  // e.g., "ValueError", "KeyError"
        message: Expression,     // Error message
        line: usize,
    },
    Expression(Expression),
    Pass,
    Import {
        path: String,
    },
    TupleUnpack {
        names: Vec<String>,
        value: Expression,
    },
}

#[derive(Debug, Clone)]
pub struct ExceptClause {
    pub exception_type: Option<String>,  // None means catch all
    pub var_name: Option<String>,        // Variable to bind exception to
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub default_value: Option<Expression>,  // Default parameter value
}

/// Represents a decorator applied to a field (e.g., @arg, @option)
#[derive(Debug, Clone)]
pub struct Decorator {
    pub name: String,                    // "arg" or "option"
    pub args: HashMap<String, String>,   // Named arguments like help="...", short="v"
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: Type,
    pub decorators: Vec<Decorator>,      // Decorators on this field
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
        named_args: Vec<(String, Expression)>,  // Named arguments: (name, value) pairs
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
    TupleLiteral {
        elements: Vec<Expression>,
    },
    TupleIndex {
        tuple: Box<Expression>,
        index: usize,             // Compile-time index (0, 1, 2, etc.)
        line: usize,
    },
    Slice {
        object: Box<Expression>,
        start: Option<Box<Expression>>,   // None = from beginning
        end: Option<Box<Expression>>,     // None = to end
        step: Option<Box<Expression>>,    // None = step of 1
        line: usize,
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
