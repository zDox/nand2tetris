use std::{
    path::Path,
    fs::read_to_string,
    str::Lines,
};
use core::iter::Peekable;


pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
    None,
}

pub struct Parser<'a> {
    content: String,
    line_iter: Box<Peekable<Lines<'a>>>,
    current_line: String,
    current_command: CommandType,
}

#[derive(Debug)]
pub enum ParseError {
    FileType,
    FileOpen,
    UnknownCommand,
    ExtractionArg1,
    ExtractionArg2,
}

impl Parser<'_> {
    pub fn new(path: &Path) -> Result<Parser<'_>, ParseError> {
        if !path.extension().is_some_and(|ext| ext == "vm") {
            return Err(ParseError::FileType);
        }
        let content = read_to_string(path);
        match content {
            Ok(content_str) => Ok(Self { 
                content: content_str, 
                line_iter: Box::new(content_str.lines().peekable()), 
                current_line: String::from(""),
                current_command: CommandType::None,
            }),
            Err(_) => Err(ParseError::FileOpen),
        }
    }

    pub fn has_more_lines(&self) -> bool {
        self.line_iter.peek().is_some()
    }

    pub fn advance(&mut self) {
        while let Some(mut next_line) =self.line_iter.next() {
            self.current_line = next_line.trim().to_string(); 
            if next_line.starts_with("//") | next_line.is_empty() {
                continue;
            }
            else { break; }
        }
    }

    pub fn command_type(&mut self) -> Result<CommandType, ParseError> {
        // Missing LABEL, IF, FUNCTION, RETURN and CALL CommandType
        if self.current_line.starts_with("push") { return Ok(CommandType::Push); }
        else if self.current_line.starts_with("pop") { return Ok(CommandType::Pop); }
        else if self.current_line.starts_with("goto") { return Ok(CommandType::Goto); }
        else if ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"].iter()
            .any(|com| self.current_line.starts_with(com)) { return Ok(CommandType::Arithmetic); }
        Err(ParseError::UnknownCommand)
    }

    pub fn arg1(&self) -> Result<String, ParseError> {
        let arg1 = match self.current_command {
            CommandType::Arithmetic => self.current_line.split_whitespace().nth(0),
            _ => self.current_line.split_whitespace().nth(1),
        };
        let res = arg1.ok_or(ParseError::ExtractionArg1);
        match res {
            Ok(val) => Ok(val.to_string()),
            Err(val) => Err(val),
        }
    }

    pub fn arg2(&self) -> Result<i32, ParseError> {
        let arg2 = self.current_line.split_whitespace().nth(2);
        let res = arg2.ok_or(ParseError::ExtractionArg2);
        match res {
            Ok(val) => match val.parse() {
                Ok(i_val) => Ok(i_val),
                Err(_) => Err(ParseError::ExtractionArg2),
            },
            Err(val) => Err(val),
        }
    }
}
