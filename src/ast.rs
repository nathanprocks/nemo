#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Op, Box<Expr>),
    Number(f64),
    Name(String),
    Call(Box<Expr>, Vec<Box<Expr>>),
    Lambda(Vec<String>, Box<Expr>),
    Pull,
    Block(Vec<Box<Statement>>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    While(Box<Expr>, Box<Expr>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Slash,
    Pipe,
    Percent,
    Greater,
    Lesser,
    Equals,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Assignment(String, Box<Expr>),
    Push(Box<Expr>),
    Expr(Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Prototype {
    name: String,
    args: Vec<String>,
}
impl Prototype {
    pub fn new(name: String, args: Vec<String>) -> Prototype {
        Prototype {
            name: name,
            args: args,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Definition {
    prototype: Prototype,
    body: Box<Expr>,
}
impl Definition {
    pub fn new(prototype: Prototype, body: Box<Expr>) -> Definition {
        Definition {
            prototype: prototype,
            body: body,
        }
    }
}
