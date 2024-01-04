use std::path::Path;

#[path = "tokenizer.rs"]
mod tokenizer;
use tokenizer::{ Tokenizer, Token };

pub struct JackAnalyzer {
    path: Box<Path>,
}

impl JackAnalyzer {
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
        while tokenizer.has_more_tokens() {
            tokenizer.advance();
            println!("{}", tokenizer.token_type().to_xml());
        }
        println!("Translation of file '{}' completed", file_path.file_name().unwrap().to_str().unwrap());
    }
}
