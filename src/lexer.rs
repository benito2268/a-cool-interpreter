use std::process;

pub mod token;

pub struct Lexer {
    pub tokens: Vec<Box<dyn token::Token>>,
    source: String,
    curr: usize,
    start: usize,
    line: usize,
    is_eof: bool,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            tokens: Vec::new(),
            source,
            curr: 0,
            start: 0,
            line: 1,
            is_eof: false,
        }
    }

    pub fn run_lexer(&mut self) {
        while !self.is_eof {
            self.start = self.curr;
            self.next();
        }

        self.add_token(token::TokenType::EOF, None::<i32>);
    }

    // returns the next token in source code
    fn next(&mut self) {
        let c = match self.next_char() {
            Some(c) => c,
            None => {
                self.is_eof = true;
                return
            }
        };

        //match operators and one char tokens
        //should probably add a error clause for illegal tokens
        match c {
            ';'  => self.add_token(token::TokenType::SEMICOL, None::<i32>),
            '+'  => self.add_token(token::TokenType::PLUS, None::<i32>),
            '-'  => self.add_token(token::TokenType::MINUS, None::<i32>),
            '*'  => self.add_token(token::TokenType::MULT, None::<i32>),
            '{'  => self.add_token(token::TokenType::LBRACK, None::<i32>),
            '}'  => self.add_token(token::TokenType::RBRACK, None::<i32>),
            '('  => self.add_token(token::TokenType::LPAREN, None::<i32>),
            ')'  => self.add_token(token::TokenType::RPAREN, None::<i32>),
            '"'  => self.string(),
            '\n' => self.line += 1,
            ' ' | '\r' |'\t' => (),
            '='  => {
                if self.expect('=') {
                    self.add_token(token::TokenType::EQUAL, None::<i32>);
                    self.next_char();
                }
                else {
                    self.add_token(token::TokenType::ASSIGN, None::<i32>);
                }
            },
            '>'  => {
                if self.expect('=') {
                    self.add_token(token::TokenType::GREATEQ, None::<i32>);
                    self.next_char();
                }
                else {
                    self.add_token(token::TokenType::GREATER, None::<i32>);
                }
            },
            '<'  => {
                if self.expect('=') {
                    self.add_token(token::TokenType::LESSEQ, None::<i32>);
                    self.next_char();
                }
                else {
                    self.add_token(token::TokenType::LESS, None::<i32>);
                }
            },
            '/' => {
                if self.expect('/') {
                    while self.peek_char().unwrap() != '\n' {
                        self.next_char();
                    }
                    self.line += 1;
                }
                else {
                    self.add_token(token::TokenType::DIV, None::<i32>);
                }
            },
            _ => {
                if c.is_alphabetic() {
                   self.identifier(); 
                }
                else if c.is_numeric() {
                    self.number();
                }
                else {
                    self.error("unrecognized character");
                }
            }
        }
    }

    fn add_token<T>(&mut self, toktype: token::TokenType, literal: Option<T>)
    where
        T: 'static,
    {
        let tokstr = &self.source[self.start..self.curr];
    
        self.tokens.push(
            match literal {
                Some(literal) => Box::new(token::LiteralToken::new(toktype, tokstr.to_string(), self.line, literal)),
                None => Box::new(token::RegToken::new(toktype, tokstr.to_string(), self.line)),
            },
        );
    }
    
    fn keyword(&self, literal: &str) -> Option<token::TokenType> {
        match literal {
            "let" => Some(token::TokenType::LET),
            "print" => Some(token::TokenType::PRINT),
            "input" => Some(token::TokenType::INPUT),
            "true" => Some(token::TokenType::TRUE),
            "false" => Some(token::TokenType::FALSE),
            "if" => Some(token::TokenType::IF),
            "elif" => Some(token::TokenType::ELIF),
            "else" => Some(token::TokenType::ELSE),
            "while" => Some(token::TokenType::WHILE),
            "for" => Some(token::TokenType::FOR),
            "class" => Some(token::TokenType::CLASS),
            "func" => Some(token::TokenType::FUNC),
            "return" => Some(token::TokenType::RETURN),
            "none" => Some(token::TokenType::NONE),
            _ => None
        }
    }

    fn identifier(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() {
                self.next_char();
            } 
            else { break; }
        }

        let tokstr = &self.source[self.start..self.curr];
        let toktype = match self.keyword(tokstr) {
            Some(toktype) => toktype,
            None => token::TokenType::IDENT,
        };

        self.add_token(toktype, None::<i32>);
    }

    fn number(&mut self) {
        let mut is_f64 = false;

        while let Some(c) = self.peek_char() {
            if c.is_numeric() {
                self.next_char();
            }
            else if c == '.' {
                is_f64 = true;
                self.next_char();
            }
            else { break; }
        }

        let tokstr: &str = &self.source[self.start..self.curr];

        if is_f64 {
            let float = String::from(tokstr).parse::<f64>().unwrap();
            self.add_token(token::TokenType::LITERAL, Some(float));
        }
        else {
            let int = String::from(tokstr).parse::<i32>().unwrap();
            self.add_token(token::TokenType::LITERAL, Some(int));
        }
    }

    fn string(&mut self) {
        loop {
            match self.peek_char() {
                Some(c) => {
                    if c == '"' {
                        break;
                    }
                    else { self.next_char(); }
                },
                None => self.error("unterminated string literal"),
            }
        }

        // go past the closing "
        self.next_char();

        let tokstr: &str = &self.source[(self.start + 1)..(self.curr - 1)];
        self.add_token(token::TokenType::LITERAL, Some(tokstr.to_string()));
    }

    fn error(&self, message: &str) {
        eprintln!("error! {} on line {}", message, self.line);
        process::exit(1);
    }

    // returns the next char in the source code
    //TODO fixme
    fn next_char(&mut self) -> Option<char> {
        if self.curr >= self.source.len() {
            None
        } else {
            let c = self.source[self.curr..].chars().next();
            self.curr += 1;
            c
        }
    }
    
    // Returns the next char without advancing the lexer
    fn peek_char(&self) -> Option<char> {
        if self.curr >= self.source.len() {
            None
        } else {
            self.source[self.curr..].chars().next()
        }
    }

    // returns true if the char at self.curr matches 'expect'
    fn expect(&self, expect: char) -> bool {
        if self.source.chars().nth(self.curr).unwrap() == expect {
            true
        }
        else { false }
    }
}
