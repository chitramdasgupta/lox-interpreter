use crate::lexical_analysis::token::Token;
use crate::syntax_analysis::expression::{Expr, Variable};

/// Statements do something like variable declarations, control flow, etc.
/// They make up an Abstract Syntax Tree (AST)
#[allow(unused)]
pub enum Stmt {
    /// Code blocks
    Block { statements: Vec<Stmt> },
    /// Class declarations
    Class {
        name: Token,
        superclass: Option<Variable>,
        methods: Vec<Function>,
    },
    /// Expression statements to handle expressions like: a + b; foo();
    Expression { expr: Expr },
    /// Function declarations
    Function(Function),
    /// If statements
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    /// Print statements
    Print { expr: Expr },
    /// Return statements inside a function body
    Return { keyword: Token, value: Expr },
    /// Variable declarations
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    /// While loops
    While { condition: Expr, body: Box<Stmt> },
}

pub struct Function {
    pub name: Token,
    pub arguments: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(fun {} (", self.name.lexeme)?;

        for (i, param) in self.arguments.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", param.lexeme)?;
        }

        write!(f, ") ")?;

        for stmt in &self.body {
            write!(f, "{}", stmt)?;
        }

        write!(f, ")")
    }
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Block { statements } => {
                write!(f, "(block")?;
                for stmt in statements {
                    write!(f, " {}", stmt)?;
                }
                write!(f, ")")
            }
            Stmt::Class {
                name,
                superclass,
                methods,
            } => {
                write!(f, "(class {}", name.lexeme)?;
                if let Some(superclass) = superclass {
                    write!(f, " < {}", superclass)?;
                }
                for method in methods {
                    write!(f, " {}", method)?;
                }
                write!(f, ")")
            }
            Stmt::Expression { expr } => {
                write!(f, "(; {})", expr)
            }
            Stmt::Function(function) => {
                write!(f, "(fun {}", function.name.lexeme)?;
                write!(f, "(")?;
                for (i, param) in function.arguments.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", param.lexeme)?;
                }
                write!(f, ") ")?;
                for stmt in &function.body {
                    write!(f, "{}", stmt)?;
                }
                write!(f, ")")
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if let Some(else_branch) = else_branch {
                    write!(f, "(if-else {} {} {})", condition, then_branch, else_branch)
                } else {
                    write!(f, "(if {} {})", condition, then_branch)
                }
            }
            Stmt::Print { expr } => {
                write!(f, "(print {})", expr)
            }
            Stmt::Return { value, .. } => {
                write!(f, "(return {})", value)
            }
            Stmt::Var { name, initializer } => {
                if let Some(init) = initializer {
                    write!(f, "(var {} = {})", name.lexeme, init)
                } else {
                    write!(f, "(var {})", name.lexeme)
                }
            }
            Stmt::While { condition, body } => {
                write!(f, "(while {} {})", condition, body)
            }
        }
    }
}
