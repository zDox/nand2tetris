#[path = "parser.rs"]
mod parser;
use parser::Parser;
#[path = "code_writer.rs"]
mod code_writer;
use code_writer::CodeWriter;

pub struct VMTranslator {
    parser: Parser,
    code_writer: CodeWriter,
}

impl VMTranslator {
}
