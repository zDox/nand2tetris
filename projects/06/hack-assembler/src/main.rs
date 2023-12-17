
mod parser;
use parser::Parser;
use std::{ env, path::{ Path, PathBuf } , fs::write};
fn main() {    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let path_arg = &args[1];

    let mut path_buf = PathBuf::from(path_arg);
    let path = Path::new(&path_buf);

    let mut parser = Parser::new();
    let byte_code: String  = parser.parse(&path).unwrap();
    path_buf.set_extension("hack");

    write(path_buf, byte_code).expect("Could not write output to file");
}
