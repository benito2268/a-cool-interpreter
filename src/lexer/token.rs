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

pub trait Token { 
    fn print(&self);
}

pub struct RegToken {
    toktype: TokenType,
    lexeme: String,
    line: usize,
}

pub struct LiteralToken<T> {
    token: RegToken,
    literal: T,
}

impl RegToken {
    pub fn new(toktype: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            toktype,
            lexeme,
            line,
        }
    }
}

impl<T> LiteralToken<T> {
    pub fn new(toktype: TokenType, lexeme: String, line: usize, literal: T) -> Self {
        Self {
            token: RegToken::new(toktype, lexeme, line),
            literal,
        }
    }
}

impl Token for RegToken { 
    fn print(&self) {
        println!("{:?}", self.toktype);
    }
}
impl<T> Token for LiteralToken<T> { 
    fn print(&self) {
        println!("{:?}", self.token.toktype);
    }
}
