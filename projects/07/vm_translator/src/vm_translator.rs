use std::path::Path;

#[path = "parser.rs"]
mod parser;
use parser::{ Parser, CommandType };
#[path = "code_writer.rs"]
mod code_writer;
use code_writer::CodeWriter;

pub struct VMTranslator {
    path: Box<Path>,
    parser: Parser,
    code_writer: CodeWriter,
}

impl VMTranslator {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.into(),
            parser: Parser::new(path).expect("File has wrong Filetype: expeceted .vm"), 
            code_writer: CodeWriter::new(),
        }
    }

    pub fn translate(&mut self) {
        while self.parser.has_more_lines() {
            self.parser.advance();
            let _res = match self.parser.command_type().expect("Failed determining CommandType"){
                CommandType::Pop => self.code_writer.write_pop(self.parser.arg1().expect("No arg1 to pop command"), self.parser.arg2().expect("No second arg to pop command")),
                CommandType::Push => self.code_writer.write_push(self.parser.arg1().expect("No arg1 to push command"), self.parser.arg2().expect("No second arg to push command")),
                CommandType::Arithmetic => self.code_writer.write_arithmetic(self.parser.arg1().expect("No Arithmetic command")),
                _ => todo!(),
            };
        }

        let _res = self.code_writer.save(&self.path);
    }
}
