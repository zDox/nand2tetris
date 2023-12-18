use std::path::Path;

#[path = "parser.rs"]
mod parser;
use parser::Parser;
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
}
