use std::{
    path::Path
};

#[path = "parser.rs"]
mod parser;
use parser::{ Parser, CommandType };
#[path = "code_writer.rs"]
mod code_writer;
use code_writer::CodeWriter;

pub struct VMTranslator {
    path: Box<Path>,
    code_writer: CodeWriter,
}

impl VMTranslator {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.into(),
            code_writer: CodeWriter::new(path.file_stem().unwrap().to_str().unwrap()),
        }
    }

    pub fn translate(&mut self) {
        if self.path.is_dir() { self.translate_folder(); }
        else if self.path.is_file() { self.translate_file(); }
    }

    fn translate_folder(&mut self) {
        let paths = self.path.read_dir().expect("Could not read folder");

        self.code_writer.write_bootstrap_code();

        for path in paths {
            let path_path = path.unwrap().path().as_path();
            if path_path.extension().unwrap() == ".asm" {
                self.code_writer.set_file_name(path_path.file_stem().unwrap().to_str().unwrap());
                self.translate_file();
            }
        }
    }


    fn translate_file(&mut self) {
        let mut parser = Parser::new(&self.path).expect("File has wrong Filetype: expeceted .vm");
        while parser.has_more_lines() {
            parser.advance();
            let _res = match parser.command_type().expect("Failed determining CommandType"){
                CommandType::Pop => self.code_writer.write_pop(
                    parser.arg1().expect("No segement specified when using pop command"), 
                    parser.arg2().expect("No index specified when using pop command")),
                CommandType::Push => self.code_writer.write_push(
                    parser.arg1().expect("No segment specified when usign push command"), 
                    parser.arg2().expect("No index specified when using push command")),
                CommandType::Arithmetic => self.code_writer.write_arithmetic(parser.arg1().expect("No Arithmetic command")),
                CommandType::Label => self.code_writer.write_label(parser.arg1().expect("No Label specified when using label command")),
                CommandType::Goto => self.code_writer.write_goto(parser.arg1().expect("No Label specified when using goto command")),
                CommandType::IfGoto => self.code_writer.write_if_goto(parser.arg1().expect("No Label specified when using if-goto command")),
                CommandType::Function => self.code_writer.write_function(
                    parser.arg1().expect("No function name specified when using function command"),
                    parser.arg2().expect("No amount of variables specified when using function command")),
                CommandType::Call => self.code_writer.write_call(
                    parser.arg1().expect("No function name specified when using call command"),
                    parser.arg2().expect("No amount of arguments specified when using call command")),
                CommandType::Return => self.code_writer.write_return(),
                CommandType::None => unreachable!(),
            };
        }

        let _res = self.code_writer.save(&self.path);
    }
}
