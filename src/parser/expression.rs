use crate::lexer::token;
use crate::lexer::literal;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: token::Token,
        right: Box<Expr>,
    },
    Unary {
        operator: token::Token,
        right: Box<Expr>,
    },
    Grouping { expr: Box<Expr> },
    Literal  { value: literal::Literal },
}

impl Expr {
    fn new_binary(left: Expr, operator: token::Token, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    fn new_unary(operator: token::Token, right: Expr) -> Self {
        Expr::Unary {
            operator,
            right: Box::new(right),
        }
    }

    fn new_grouping(expr: Expr) -> Self {
        Expr::Grouping {
            expr: Box::new(expr)
        }
    }

    fn new_literal(value: literal::Literal) -> Self {
        Expr::Literal { value }
    }

    
}