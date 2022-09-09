use logos::{Logos, Lexer};

pub fn lex(source: &str) -> Vec<Token> {
    Token::lexer(source).collect()
}

#[derive(Debug, Clone, Logos, PartialEq)]
pub enum Token {
    #[token("&")]
    Ampersand,
    #[token("&&")]
    And,
    #[token("=")]
    Assign,
    #[token("->")]
    Arrow,
    #[token("|")]
    Bar,
    #[token("}")]
    CloseBrace,
    #[token(")")]
    CloseParen,
    #[token(":")]
    Colon,
    #[token("\0")]
    Eof,
    #[token("==")]
    Eq,
    #[token("fn")]
    Fn,
    #[token(">=")]
    Geq,
    #[token(">")]
    Gt,
    #[regex(r"[a-zA-Z_?]+", to_string)]
    Ident(String),
    #[token("int")]
    Int,
    #[regex(r"[0-9]+", to_int)]
    IntLit(i64),
    #[token("<=")]
    Leq,
    #[token("<<")]
    LShift,
    #[token("<")]
    Lt,
    #[token("-")]
    Minus,
    #[token("--")]
    MinusMinus,
    #[token("!=")]
    Neq,
    #[token("{")]
    OpenBrace,
    #[token("(")]
    OpenParen,
    #[token("||")]
    Or,
    #[token("%")]
    Percent,
    #[token("+")]
    Plus,
    #[token("++")]
    PlusPlus,
    #[token("?")]
    Question,
    #[token("return")]
    Return,
    #[token(">>")]
    RShift,
    #[token(";")]
    Semicolon,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,
    #[regex(r##""(?:[^"\\]|\\.)*""##, to_string)]
    StringLit(String),
    #[token("^")]
    Xor,

    #[error]
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn to_string(lex: &mut Lexer<Token>) -> Option<String> {
    let mut str = lex.slice().to_string();

    if str.starts_with("\"") {
        str.remove(0);
    }

    if str.ends_with('"') {
        str.remove(str.len() - 1);
    }

    Some(str)
}

fn to_int(lex: &mut Lexer<Token>) -> Option<i64> {
    Some(lex.slice().parse().ok()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_comments() {
        let mut lexer = Token::lexer("// test");

        assert_eq!(lexer.next(), None);
    }
}