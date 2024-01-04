use std::{
    path::Path,
    fs::read_to_string,
    fmt,
};


pub enum Token {
    Keyword(String),
    Symbol(char),
    IntegerConstant(u16),
    StringConstant(String),
    Identifier(String),
    None,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
            Token::Keyword(s) => write!(f, "Keyword: {}", s),
            Token::Symbol(s) => write!(f, "Symbol: {}", s),
            Token::IntegerConstant(s) => write!(f, "IntegerConstant: {}", s),
            Token::StringConstant(s) => write!(f, "StringConstant: {}", s),
            Token::Identifier(s) => write!(f, "Identifier: {}", s),
            Token::None => write!(f, "None"),
        }
    }
}


pub struct Tokenizer {
    content: String,
    index: u32,
    current_line: String,
    current_token: Token,
}

#[derive(Debug)]
pub enum TokenError {
    FileType,
    FileOpen,
    UnknownToken,
}

impl Tokenizer {
    pub fn new(path: &Path) -> Result<Tokenizer, TokenError> {
        if !path.extension().is_some_and(|ext| ext == "jack") {
            return Err(TokenError::FileType);
        }
        let content = read_to_string(path).map_err(|_| TokenError::FileOpen)?;

        Ok(Self { 
            content, 
            index: 0, 
            current_line: String::from(""),
            current_token: Token::None,
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
}
