use std::fs::File;
use std::io::Read;
use std::path::Path;

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
    let lines = text.split("\n");

    for line in lines {
        println!("{}", line);
    }
}
