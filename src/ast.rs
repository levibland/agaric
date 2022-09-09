use crate::lexer::Token;

pub type Program = Vec<Statement>;
pub type Block = Vec<Statement>;
pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Return {
        value: Expression,
    },
    FuncDeclaration {
        name: Identifier,
        params: Vec<Parameter>,
        body: Block,
    },
    LetDeclaration {
        name: Identifier,
        initial: Option<Expression>,
    },
    Expression {
        expression: Expression,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Integer(i64),
    String(String),
    Identifier(Identifier),
    Assign(Box<Expression>),
    Infix(Box<Expression>, Op, Box<Expression>),
    Prefix(Op, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    Assign,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    PlusPlus,
    MinusMinus,
}

impl Op {
    pub fn token(token: Token) -> Self {
        match token {
            Token::Plus => Self::Add,
            Token::Minus => Self::Subtract,
            Token::Star => Self::Multiply,
            Token::Slash => Self::Divide,
            Token::Percent => Self::Modulo,
            Token::Eq => Self::Equals,
            Token::Neq => Self::NotEquals,
            Token::Assign => Self::Assign,
            Token::Lt => Self::LessThan,
            Token::Gt => Self::GreaterThan,
            Token::Leq => Self::LessThanEquals,
            Token::Geq => Self::GreaterThanEquals,
            Token::PlusPlus => Self::PlusPlus,
            Token::MinusMinus => Self::MinusMinus,
            _ => unreachable!("{:?}", token),
        }
    }
}