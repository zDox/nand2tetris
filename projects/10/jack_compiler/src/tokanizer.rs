use std::{
    path::Path,
    fs::read_to_string,
    fmt,
};

static KEYWORDS: [&str; 21] = [
    "class", "method", "function", "constructor", "int", "boolean", 
    "char", "void", "var", "static", "field", "let", "do", "if", 
    "else", "while", "return", "true", "false", "null", "this"
];

static SYMBOLS: [char; 19] = [
    '{', '}', '(', ')', '[', ']', '.', ',',
    ';', '+', '-', '*', '/', '&', '|', '<', '>',
    '=', '~',
];

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
            Token::Symbol(c) => write!(f, "Symbol: {}", c),
            Token::IntegerConstant(int) => write!(f, "IntegerConstant: {}", int),
            Token::StringConstant(s) => write!(f, "StringConstant: {}", s),
            Token::Identifier(s) => write!(f, "Identifier: {}", s),
            Token::None => write!(f, "None"),
        }
    }
}


pub struct Tokenizer {
    content: String,
    index: u32,
    current_str: String,
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
            current_str: String::from(""),
            current_token: Token::None,
        })
    }

    fn peek(&self) -> Option<char> {
        self.content.chars().nth(self.index.try_into().unwrap())
    }

    pub fn has_more_tokens(&self) -> bool {
        self.peek().is_some()
    }

    pub fn advance(&mut self) {
        loop {
            let c = self.content.chars().nth(self.index.try_into().unwrap()).unwrap();
            
            // whitespace
            if c.is_whitespace() {
                self.index += 1;
                continue 
            }
            
            // StringConstant
            if c == '"' {
                let end_index = self.content[self.index as usize..]
                    .find(|t| t == '"')
                    .map(|i| self.index as usize + i)
                    .unwrap_or(self.content.len());

                let result = &self.content[self.index as usize..end_index];
                self.current_token = Token::StringConstant(result.to_string());
                self.index = end_index as u32 + 1;
            }

            // if current char is digit and next char than it has to be a IntegerConstant
            else if c.is_digit(10) {

                let end_index = self.content[self.index as usize..]
                    .find(char::is_whitespace)
                    .map(|i| self.index as usize + i)
                    .unwrap_or(self.content.len());

                let result = &self.content[self.index as usize..end_index];

                self.current_token = Token::IntegerConstant(result.parse().unwrap());
                self.index = end_index as u32 + 1;
            }

            else if SYMBOLS.contains(&c) {
                self.current_token = Token::Symbol(c);
                self.index += 1;
            }

            else {
                // Else its either a keyword or a identifier
                let end_index = self.content[self.index as usize..]
                    .find(char::is_whitespace)
                    .map(|i| self.index as usize + i)
                    .unwrap_or(self.content.len());

                let result = &self.content[self.index as usize..end_index];

                self.current_str = result.to_string();
                if KEYWORDS.contains(&result) {
                    self.current_token = Token::Keyword(result.to_string());
                }
                else {
                    self.current_token = Token::Identifier(result.to_string());
                }

                self.index = end_index as u32 + 1;
            }
            return 
        }
    }

    pub fn token_type(&self) -> &Token{
        return &self.current_token;
    }

    pub fn keyword(&self) {
    }

    pub fn symbol(&self) {
    }

    pub fn identifier(&self) {
    }

    pub fn int_val(&self) {
    }

    pub fn string_val(&self) {
    }
}
