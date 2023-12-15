use std::{
    fs::read_to_string,
    path::Path,
    fmt,
};

#[derive(Debug)]
pub enum Token {
    // Elements of A instruction
    Symbol(String),
    Label(String),

    // Elements of C instruction
    Dest(String),
    Comp(String),
    Jump(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
            Token::Symbol(s) => write!(f, "Symbol: {}", s),
            Token::Label(s) => write!(f, "Label: {}", s),
            Token::Dest(s) => write!(f, "Dest: {}", s),
            Token::Comp(s) => write!(f, "Comp: {}", s),
            Token::Jump(s) => write!(f, "Jump: {}", s),
        }
    }
}

#[derive(Debug)]
pub enum InstructionType {
    A,
    C,
    L,
}

#[derive(Debug)]
pub enum ParseError {
    FileType,
    InvalidInstruction,
}

pub type Tokens = Vec<Vec<Token>>;

pub struct Parser {
}

impl Parser {
    pub fn new() -> Self {
        Parser{}
    }
    pub fn parse(path: &Path) -> Result<Tokens, ParseError> {
        let mut tokens: Tokens = vec!();

        // return Err if extension is not asm
        if !path.extension().is_some_and(|value| value == "asm") {
            return Err(ParseError::FileType);
        }

        let content = read_to_string(path).unwrap();

        for line in content.split("\n") {
            let line_tokens = Self::parse_line(line).unwrap();
            tokens.push(line_tokens);
        }

        Ok(tokens)
    }

    fn parse_line(line: &str) -> Result<Vec<Token>, ParseError> {
        let mut tokens: Result<Vec<Token>, ParseError> = Ok(vec!());
        tokens = Self::parse_a_inst(line);
        if tokens.is_ok() {
            return Ok(tokens.unwrap());
        }

        tokens = Self::parse_l_inst(line);
        if tokens.is_ok() {
            return Ok(tokens.unwrap());
        }

        tokens = Self::parse_c_inst(line);
        if tokens.is_ok() {
            return Ok(tokens.unwrap());
        }
        Err(ParseError::InvalidInstruction)
    }

    fn parse_a_inst(line: &str) -> Result<Vec<Token>, ParseError> {
        let words: Vec<&str> = line.trim().split_whitespace().collect();
        if words.len() == 1 && words[0].starts_with("@") {
            return Ok(vec!(Token::Symbol(words[0].get(0..).unwrap().to_string())));
        }
        

        Err(ParseError::InvalidInstruction)
    }

    fn parse_l_inst(line: &str) -> Result<Vec<Token>, ParseError> {
        if line.trim().chars().all(|c| c.is_alphanumeric()) {
            return Ok(vec!(Token::Label(line.trim().to_string())));
        }
        Err(ParseError::InvalidInstruction)
    }

    fn parse_c_inst(line: &str) -> Result<Vec<Token>, ParseError> {
        let line = line.trim();
        let mut current = String::from("");
        let mut c_iter = line.chars();
        let mut tokens: Vec<Token> = vec!();
        // parse dest
        while let Some(c) = c_iter.next() {
            if c == '=' {
                if current.is_empty() {
                    return Err(ParseError::InvalidInstruction);
                }
                tokens.push(Token::Dest(current.clone()));
                continue;
            }
            if c == ';' {
                tokens.push(Token::Comp(current.clone()));
                continue;
            }
            if c.is_alphanumeric() && current.chars().last().is_some_and(|last_char| last_char.is_whitespace()) {
                return Err(ParseError::InvalidInstruction);
            }
            if c.is_alphanumeric() {
                current.push(c);
                continue;
            }
            if c.is_whitespace() {
                continue;
            }
            return Err(ParseError::InvalidInstruction);
        }

        if !current.is_empty() {
            tokens.push(Token::Jump(current.clone()));
        }

        Ok (tokens)
    }
}
