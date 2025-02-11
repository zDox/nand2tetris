use std::path::Path;

#[path = "tokenizer.rs"]
mod tokenizer;
use tokenizer::{ Tokenizer, Token };

#[path = "compilation_engine.rs"]
mod compilation_engine;
use compilation_engine::CompilationEngine;

#[path = "symbol_table.rs"]
mod symbol_table;
use symbol_table::SymbolTable;


pub struct JackCompiler {
    path: Box<Path>,
}

impl JackCompiler {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.into(),
        }
    }

    pub fn compile(&mut self) {
        if self.path.is_dir() { 
            self.compile_folder(); 
        }
        else if self.path.is_file() { self.compile_file(&self.path.clone()); } 
    }

    fn compile_folder(&mut self) {
        println!("Compiling folder '{}'", self.path.display());
        let paths = self.path.read_dir().expect("Could not read folder");

        for path in paths {
            let file_path = path.unwrap().path();
            if file_path.extension().unwrap() == "jack" {
                self.compile_file(&file_path);
            }
        }
    }

    fn compile_file(&mut self, file_path: &Path) {
        let mut tokenizer = Tokenizer::new(file_path).expect("File has wrong Filetype: expeceted .jack");
        let mut tokens: Vec<Token> = vec!();

        while tokenizer.has_more_tokens() {
            tokenizer.advance();
            tokens.push(tokenizer.token_type().clone());
        }



        let mut path_buf = file_path.to_path_buf();
        path_buf.set_file_name(&format!("{}.vm", file_path.file_stem().unwrap().to_str().unwrap()));
        let mut compilation_engine = CompilationEngine::new(tokens, &path_buf);
        
        compilation_engine.run();
        println!("Compilation of file '{}' completed", file_path.file_name().unwrap().to_str().unwrap());
    }
}
