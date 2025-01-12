use crate::lexer::literal;

#[derive(Debug)]
pub enum TokenType {
    LET, PRINT, TRUE, FALSE, IF, ELSE, ELIF,
    WHILE, FOR, RETURN, FUNC, CLASS, NONE,
    INPUT,

    PLUS, MINUS, MULT, DIV, LPAREN, RPAREN,
    LBRACK, RBRACK, EQUAL, GREATER, LESS,
    ASSIGN, GREATEQ, LESSEQ,

    IDENT, LITERAL, SEMICOL, EOF,
}

pub struct Token {
    toktype: TokenType,
    lexeme: String,
    line: usize,
    literal: Option<literal::Literal>,
}

impl Token {
    pub fn new(toktype: TokenType, lexeme: String, line: usize, literal: Option<literal::Literal>) -> Self {
        Self {
            toktype,
            lexeme,
            line,
            literal,
        }
    }
}

impl Token {
    pub fn print(&self) {
        match &self.literal {
            Some(literal) => println!("{:?} : {}", self.toktype, literal),
            None => println!("{:?}", self.toktype),
        }
    }
}
