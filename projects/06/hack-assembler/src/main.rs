
mod parser;
use parser::{ Parser, ParseError, Tokens};
use std::{ env, path::{ Path, PathBuf } };
fn main() {    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let path_arg = &args[1];

    let path_buf = PathBuf::from(path_arg);
    let path = Path::new(&path_buf);

    let parser: Parser = Parser::new();
    let tokens: Tokens = Parser::parse(&path).unwrap();
    tokens.iter().for_each(|line| {
        line.iter().for_each(|token| {
            print!("{}, ", token);
        });
        println!();
    });
}
