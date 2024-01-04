use std::path::Path;
use std::fs::write;

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
        let mut content = String::from("");
        content.push_str("<tokens>\n");

        while tokenizer.has_more_tokens() {
            tokenizer.advance();
            println!("{}", tokenizer.token_type().to_xml());
            content.push_str(&format!("{}\n", tokenizer.token_type().to_xml()));
            
        }

        content.push_str("</tokens>");
        let mut path_buf = file_path.to_path_buf();
        path_buf.set_file_name(&format!("{}TMine.xml", file_path.file_stem().unwrap().to_str().unwrap()));
        
        write(path_buf, content).expect("Couldnt write xml file");
        println!("Translation of file '{}' completed", file_path.file_name().unwrap().to_str().unwrap());
    }
}
