
use std::{ env, path::Path };

mod vm_translator;
use vm_translator::VMTranslator;

fn main() {    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let path_arg = &args[1];

    let path = Path::new(&path_arg);
    let mut vm_translator = VMTranslator::new(&path);
    vm_translator.translate();

}
