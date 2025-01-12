use std::fmt;

#[derive(Debug)]
pub enum Literal {
    Int { val: i32 },
    Float { val: f64 },
    Bool { val : bool },
    String { val: String },
    Ident {  val: String },
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Int{val} => write!(f, "{}", val),
            Literal::Float{val} => write!(f, "{}", val),
            Literal::Bool{val} => write!(f, "{}", val),
            Literal::String{val} => write!(f, "{}", val),
            Literal::Ident{val} => write!(f, "{}", val),
        }
    }
}