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
                CommandType::Pop => self.code_writer.write_pop(
                    self.parser.arg1().expect("No segement specified when using pop command"), 
                    self.parser.arg2().expect("No index specified when using pop command")),
                CommandType::Push => self.code_writer.write_push(
                    self.parser.arg1().expect("No segment specified when usign push command"), 
                    self.parser.arg2().expect("No index specified when using push command")),
                CommandType::Arithmetic => self.code_writer.write_arithmetic(self.parser.arg1().expect("No Arithmetic command")),
                CommandType::Label => self.code_writer.write_label(self.parser.arg1().expect("No Label specified when using label command")),
                CommandType::Goto => self.code_writer.write_goto(self.parser.arg1().expect("No Label specified when using goto command")),
                CommandType::IfGoto => self.code_writer.write_if_goto(self.parser.arg1().expect("No Label specified when using if-goto command")),
                CommandType::Function => self.code_writer.write_function(
                    self.parser.arg1().expect("No function name specified when using function command"),
                    self.parser.arg2().expect("No amount of variables specified when using function command")),
                CommandType::Call => self.code_writer.write_call(
                    self.parser.arg1().expect("No function name specified when using call command"),
                    self.parser.arg2().expect("No amount of arguments specified when using call command")),
                CommandType::Return => self.code_writer.write_return(),
                CommandType::None => unreachable!(),
            };
        }

        let _res = self.code_writer.save(&self.path);
    }
}
