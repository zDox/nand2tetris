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

#[derive(Clone, PartialEq, Eq)]
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

impl Token {
    pub fn to_xml(&self) -> String {
        match &self {
            Token::Keyword(s) => format!("<keyword> {} </keyword>", s),
            Token::Symbol(c) => {
                let res: String = match c {
                    '<' => "&lt;".to_string(),
                    '>' => "&gt;".to_string(),
                    '"' => "&quot;".to_string(),
                    '&' => "&amp;".to_string(),
                    val => val.to_string(),
                };
                format!("<symbol> {} </symbol>", res)
            },
            Token::IntegerConstant(i) => format!("<integerConstant> {} </integerConstant>",i),
            Token::StringConstant(s) => format!("<stringConstant> {} </stringConstant>",s),
            Token::Identifier(s) => format!("<identifier> {} </identifier>",s),
            Token::None => "".to_string(),
        }
    }

    pub fn equals_type(&self, token2: &Token) -> bool {
        match (self, token2) {
            (Token::Keyword(_), Token::Keyword(_))
            | (Token::Symbol(_), Token::Symbol(_))
            | (Token::IntegerConstant(_), Token::IntegerConstant(_))
            | (Token::StringConstant(_), Token::StringConstant(_))
            | (Token::Identifier(_), Token::Identifier(_))
            | (Token::None, Token::None) => true,
            _ => false,
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
        self.content.chars().nth(self.index as usize)
    }

    pub fn has_more_tokens(&self) -> bool {
        self.peek().is_some()
    }

    pub fn advance(&mut self) {
        loop {
            let a = self.content.chars().nth(self.index as usize);
            if a.is_none() {
                self.current_token = Token::None;
                return;
            }
            let c = a.unwrap();
            
            // whitespace
            if c.is_whitespace() || c == '\n' {
                self.index += 1;
                continue 
            }


            // Comment
            if c == '/' && self.content.chars().nth(self.index as usize + 1)
                .is_some_and(|t| t == '/') {
                let end_index = self.content[self.index as usize..]
                    .find(|t| t == '\n')
                    .map(|i| self.index as usize + i)
                    .unwrap_or(self.content.len());
                println!("Line comment from {} till {}: {}", self.index, end_index, &self.content[self.index as usize..end_index]);
                self.index = end_index as u32 + 1;
                continue
            }
            else if c == '/' && self.content.chars().nth(self.index as usize +1)
                .is_some_and(|t| t == '*') {
                    let end_index = self.content[self.index as usize..].find("*/");
                    if let Some(mut end_index) = end_index {
                        end_index += self.index as usize + 2;
                        println!("Block comment from {} till {}: {}", self.index, self.index, &self.content[self.index as usize..end_index]);
                        self.index = end_index as u32 +1;
                        continue
                    }
                    println!("Block comment from {} : {}", self.index, &self.content[self.index as usize..end_index.unwrap()]);
            }
            
            // StringConstant from " till "
            if c == '"' {
                let end_index = self.content[self.index as usize +1..]
                    .find(|t| t == '"')
                    .map(|i| self.index as usize + i)
                    .unwrap_or(self.content.len());

                let result = &self.content[self.index as usize +1..end_index +1];
                self.current_token = Token::StringConstant(result.to_string());
                self.index = end_index as u32 + 2;
            }

            // IntegerConstant from first digit till a char is not a digit
            else if c.is_digit(10) {
                let end_index = self.content[self.index as usize..]
                    .find(|t: char| !t.is_digit(10))
                    .map(|i| self.index as usize + i)
                    .unwrap_or(self.content.len());

                let result = &self.content[self.index as usize..end_index];

                self.current_token = Token::IntegerConstant(result.parse().unwrap());
                self.index = end_index as u32;
            }

            // Symbol 
            else if SYMBOLS.contains(&c) {
                self.current_token = Token::Symbol(c);
                self.index += 1;
            }

            else {
                // Else its either a keyword or a identifier
                let end_index = self.content[self.index as usize..]
                    .find(|t: char| !(t.is_alphabetic() || t == '_' || t.is_digit(10)))
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

                self.index = end_index as u32;
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
