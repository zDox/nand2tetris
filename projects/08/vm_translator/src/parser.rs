use std::{
    path::Path,
    fs::read_to_string,
    fmt,
};

pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    IfGoto,
    Function,
    Return,
    Call,
    None,
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
            CommandType::Arithmetic => write!(f, "Arithmetic"),
            CommandType::Push => write!(f, "Push"),
            CommandType::Pop => write!(f, "Pop"),
            CommandType::Label => write!(f, "Label"),
            CommandType::Goto => write!(f, "Goto"),
            CommandType::IfGoto => write!(f, "If"),
            CommandType::Function => write!(f, "Function"),
            CommandType::Return => write!(f, "Return"),
            CommandType::Call => write!(f, "Call"),
            CommandType::None => write!(f, "None"),
        }
    }
}


pub struct Parser {
    content: String,
    index: u32,
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

impl Parser {
    pub fn new(path: &Path) -> Result<Parser, ParseError> {
        if !path.extension().is_some_and(|ext| ext == "vm") {
            return Err(ParseError::FileType);
        }
        let content = read_to_string(path).map_err(|_| ParseError::FileOpen)?;

        Ok(Self { 
            content, 
            index: 0, 
            current_line: String::from(""),
            current_command: CommandType::None,
        })
    }

    fn peek(&self) -> Option<&str> {
        self.content.lines().nth(self.index.try_into().unwrap()).and_then(|res| Some(res.trim()))
    }

    fn next(&mut self) -> Option<&str> {
        let res = self.content.lines().nth(self.index.try_into().unwrap());
        self.index += 1;
        res.and_then(|res| { self.current_line = res.trim().to_string(); Some(res.trim()) })
    }

    pub fn has_more_lines(&self) -> bool {
        self.peek().is_some()
    }

    pub fn advance(&mut self) {
        while let Some(next_line) = self.next() {
            println!("{}", next_line);
            if next_line.starts_with("//") | next_line.is_empty() {
                continue;
            }
            else { break; }
        }
    }

    pub fn command_type(&mut self) -> Result<&CommandType, ParseError> {
        self.current_command = match &self.current_line {
            line if line.starts_with("push") => CommandType::Push,
            line if line.starts_with("pop") => CommandType::Pop,
            line if ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"]
                .iter()
                .any(|com| line.starts_with(com)) => CommandType::Arithmetic,
            line if line.starts_with("label") => CommandType::Label,
            line if line.starts_with("goto") => CommandType::Goto,
            line if line.starts_with("if-goto") => CommandType::IfGoto,
            line if line.starts_with("function") => CommandType::Function,
            line if line.starts_with("call") => CommandType::Call,
            line if line.starts_with("return") => CommandType::Return,
            _ => return Err(ParseError::UnknownCommand),
        };

        Ok(&self.current_command)
    }

    pub fn arg1(&self) -> Result<&str, ParseError> {
        let arg1 = match self.current_command {
            CommandType::Arithmetic => self.current_line.split_whitespace().nth(0),
            _ => self.current_line.split_whitespace().nth(1),
        };
        arg1.ok_or(ParseError::ExtractionArg1)
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
