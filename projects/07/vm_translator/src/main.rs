
use std::{ env, path::{ Path, PathBuf }};

mod vm_translator;
use vm_translator::VMTranslator;

fn main() {    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let path_arg = &args[1];

    let mut path_buf = PathBuf::from(path_arg);
    let path = Path::new(&path_buf);
    let vm_translator = VMTranslator::new(&path);

    println!("Hello, world!");
}
