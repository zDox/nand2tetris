
use std::{ env, path::Path };

mod jack_compiler;
use jack_compiler::JackCompiler;

fn main() {    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let path_arg = &args[1];

    let path = Path::new(&path_arg);
    let mut jack_analyzer = JackCompiler::new(&path);
    jack_analyzer.compile();

}
