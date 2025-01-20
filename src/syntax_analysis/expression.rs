use crate::lexical_analysis::token::{Literal, Token};

/// Expressions evaluate to values and exist as part of a statement
#[allow(unused)]
pub enum Expr {
    /// Variable assignments: x = 42
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    /// Binary operations: 1 + 2
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    /// Function calls: foo()
    Call {
        callee: Box<Expr>,
        parenthesis: Token,
        arguments: Vec<Expr>,
    },
    /// Property accesses: object.name
    Get {
        object: Box<Expr>,
        name: Token,
    },
    /// Groupings: (1 + 2)
    Group {
        expression: Box<Expr>,
    },
    /// Literals: numbers, strings, and so on
    Literal {
        value: Option<Literal>,
    },
    /// Logical operations: x and y
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    /// Property assignment: object.name = value
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    /// Super class access: super.method()
    Super {
        keyword: Token,
        method: Token,
    },
    /// this keyword
    This {
        keyword: Token,
    },
    /// Unary operations: -42
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    /// Variable references: x
    Variable {
        name: Variable,
    },
}

pub struct Variable {
    pub name: Token,
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.lexeme)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Assign { name, value } => {
                write!(f, "(= {} {})", name.lexeme, value)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Call {
                callee, arguments, ..
            } => {
                write!(f, "(call {}", callee)?;
                for arg in arguments {
                    write!(f, " {}", arg)?;
                }
                write!(f, ")")
            }
            Expr::Get { object, name } => {
                write!(f, "(. {} {})", object, name.lexeme)
            }
            Expr::Group { expression } => {
                write!(f, "(group {})", expression)
            }
            Expr::Literal { value } => match value {
                None => {
                    write!(f, "nil")
                }
                Some(val) => {
                    write!(f, "{}", val)
                }
            },
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Set {
                object,
                name,
                value,
            } => {
                write!(f, "(= {} {} {})", object, name.lexeme, value)
            }
            Expr::Super { method, .. } => {
                write!(f, "(super {})", method.lexeme)
            }
            Expr::This { .. } => {
                write!(f, "this")
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
            Expr::Variable { name } => {
                write!(f, "{}", name.name.lexeme)
            }
        }
    }
}
