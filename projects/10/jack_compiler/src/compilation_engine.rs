use std::{ fs::write, path::Path };

use super::tokenizer::Token;

pub struct CompilationEngine {
    index: u32,
    tokens: Vec<Token>,
    output: String,
}

impl CompilationEngine {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { 
            index: 0,
            tokens,
            output: String::from(""), 
        }
    }

    pub fn save(&self, file_path: &Path) {
        write(file_path, &self.output).expect("Could not write output of CompilationEngine");
    }

    pub fn run(&mut self) {
        self.compile_class();
    }

    fn compile_class(&mut self) {
    }

    fn compile_class_var_dec(&mut self) {
    }

    fn compile_subroutine(&mut self) {
    }

    fn compile_parameter_list(&mut self) {
    }

    fn compile_subroutine_body(&mut self) {
    }

    fn compile_var_dec(&mut self) {
    }

    fn compile_statements(&mut self) {
    }

    fn compile_let(&mut self) {
    }

    fn compile_if(&mut self) {
    }

    fn compile_while(&mut self) {
    }

    fn compile_do(&mut self) {
    }

    fn compile_return(&mut self) {
    }

    fn compile_expression(&mut self) {
    }

    fn compile_term(&mut self) {
    }

    fn compile_expression_list(&mut self) {
    }
}
