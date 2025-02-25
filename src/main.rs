use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod lexer;
pub mod parser;

fn file2str(filename: &str) -> String {
    let path = Path::new(filename);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };

    let mut result = String::new();
    match file.read_to_string(&mut result) {
        Ok(_) => result,
        Err(why) => panic!("{}", why),
    }
}

fn main() {
    let text = file2str("./test.txt");

    let mut t = lexer::Lexer::new(text);

    t.run_lexer();

    for tok in t.tokens {
        tok.print();
    }

    println!("got here");

}
