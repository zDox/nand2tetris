use std::{ fs::write, path::Path };

use super::tokenizer::Token;

pub struct CompilationEngine {
    indent: usize,
    index: usize,
    tokens: Vec<Token>,
    output: String,
}

impl CompilationEngine {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { 
            indent: 0,
            index: 0,
            tokens,
            output: String::from(""), 
        }
    }

    pub fn save(&self, file_path: &Path) {
        write(file_path, &self.output).expect("Could not write output of CompilationEngine");
    }

    fn write_line(&mut self, line: &str) {
        self.output.push_str(&(" ".repeat(self.indent) + &line + "\n"));
    }

    fn write_xml(&mut self, tag: &str, content: &str) {
        self.write_line(&format!("<{}> {} </{}>", tag, content, tag));
    }

    fn write_tag(&mut self, tag: &str) {
        self.write_line(&format!("<{}>", tag));
    }

    fn write_end_tag(&mut self, tag: &str) {
        self.write_line(&format!("</{}>", tag));
    }

    fn eat(&mut self, to_eat_token: &Token) {
        if let Some(token) = self.next(){
            if token == to_eat_token {
                let copy = token.clone();
                self.write_line(&(copy.to_xml()));
            }
        }
        else {
            panic!("Next token is not expected");
        }
    }


    fn eat_independent(&mut self, to_eat_token: &Token) {
        if let Some(token) = self.next(){
            if token.equals_type(to_eat_token){
                let copy = token.clone();
                self.write_line(&(copy.to_xml()));
            }
        }
        else {
            panic!("Next token is not expected");
        }
    }


    fn eat_tokens(&mut self, to_eat_tokens: Vec<&Token>) {
        if let Some(token) = self.next(){
            if to_eat_tokens.contains(&token) {
                let copy = token.clone();
                self.write_line(&(copy.to_xml()));
            }
        }
        else {
            panic!("Next token is not expected");
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn next(&mut self) -> Option<&Token> {
        let next = self.tokens.get(self.index);
        self.index += 1;
        next
    }

    pub fn run(&mut self) {
        self.compile_class();
    }

    fn compile_class(&mut self) {
        self.write_tag("class");

        self.indent += 1;

        self.eat(&Token::Keyword("class".to_string()));
        self.eat(&Token::Identifier("Main".to_string()));
        self.eat(&Token::Symbol('{'));

        while self.peek().is_some_and(|next_token| 
                                      [Token::Keyword("static".to_string()), Token::Keyword("field".to_string())].contains(next_token)) {
            self.compile_class_var_dec();
        }
        while self.peek().is_some_and(|next_token| 
                                      [Token::Keyword("function".to_string()), Token::Keyword("method".to_string()), 
                                      Token::Keyword("constructor".to_string())].contains(next_token)) {
            self.compile_subroutine();
        }
        self.indent -= 1;
        self.write_end_tag("class");
    }

    fn compile_class_var_dec(&mut self) {
        self.write_tag("classVarDec");
        self.indent += 1;

        self.eat_tokens(vec!(&Token::Keyword("static".to_string()), &Token::Keyword("field".to_string()))); 
        self.eat_tokens(vec!(&Token::Keyword("boolean".to_string()), &Token::Keyword("int".to_string()), &Token::Keyword("char".to_string()))); 
        self.eat_independent(&Token::Identifier(String::from("")));

        while self.peek().is_some_and(|next_token: &Token| next_token == &Token::Symbol(',')) {
            self.eat(&Token::Symbol(','));
            self.eat_independent(&Token::Identifier(String::from("")));
        }
        self.eat(&Token::Symbol(';'));
        self.indent -= 1;
    }

    fn compile_subroutine(&mut self) {
        self.eat_tokens(vec!(&Token::Keyword("constructor".to_string()), 
                             &Token::Keyword("function".to_string()), 
                             &Token::Keyword("method".to_string()))); 

        self.eat_tokens(vec!(&Token::Keyword("void".to_string()), 
                             &Token::Keyword("boolean".to_string()), &Token::Keyword("int".to_string()), &Token::Keyword("char".to_string()))); 

        self.eat_independent(&Token::Identifier(String::from("")));

        self.compile_parameter_list();
        self.compile_subroutine_body();
    }

    fn compile_parameter_list(&mut self) {
        self.eat(&Token::Symbol('('));

        
        while self.peek().is_some_and(|next_token: &Token| [Token::Keyword("boolean".to_string()), Token::Keyword("boolean".to_string()), 
                                                            Token::Keyword("char".to_string())].contains(next_token)) {

            self.eat_tokens(vec!(&Token::Keyword("boolean".to_string()), &Token::Keyword("int".to_string()), &Token::Keyword("char".to_string()))); 
            self.eat_independent(&Token::Identifier(String::from("")));

            if self.peek().is_some_and(|next_token: &Token| next_token == &Token::Symbol(',')){
                self.eat(&Token::Symbol(','));
            }
            else {
                continue;
            }
        }


        self.eat(&Token::Symbol(')'));
    }

    fn compile_subroutine_body(&mut self) {
        self.eat(&Token::Symbol('{'));

        
        while self.peek().is_some_and(|next_token: &Token| next_token == &Token::Keyword("var".to_string())) {
            self.compile_var_dec();
        }

        self.compile_statements();

        self.eat(&Token::Symbol('}'));
    }

    fn compile_var_dec(&mut self) {

        self.eat(&Token::Keyword("var".to_string())); 
        self.eat_tokens(vec!(&Token::Keyword("boolean".to_string()), &Token::Keyword("int".to_string()), &Token::Keyword("char".to_string()))); 
        self.eat_independent(&Token::Identifier(String::from("")));

        while self.peek().is_some_and(|next_token: &Token| next_token == &Token::Symbol(',')) {
            self.eat(&Token::Symbol(','));
            self.eat_independent(&Token::Identifier(String::from("")));
        }

        self.eat(&Token::Symbol(';'));
    }

    fn compile_statements(&mut self) {
        while self.peek().is_some_and(|next_token: &Token| next_token != &Token::Keyword("return".to_string())) {
            let next_token = self.peek().expect("Ran out of tokens");
            if let Token::Keyword(keyword) = next_token{
                match keyword.as_str(){
                    "let" => self.compile_let(),
                    "if" => self.compile_if(),
                    "while" => self.compile_while(),
                    "do" => self.compile_do(),
                    _ => (),
                }
            }
        }

        self.compile_return();
    }

    fn compile_let(&mut self) {
        self.eat(&Token::Keyword("let".to_string())); 
        self.eat_independent(&Token::Identifier(String::from("")));

        if self.peek().is_some_and(|next_token| next_token == &Token::Symbol('[')) {
            self.eat(&Token::Symbol('['));
            self.compile_expression();
            self.eat(&Token::Symbol(']'));
        }

        self.eat(&Token::Symbol('='));
        self.compile_expression();
        self.eat(&Token::Symbol(';'));
    }

    fn compile_if(&mut self) {
        self.eat(&Token::Keyword("if".to_string())); 
        self.eat(&Token::Symbol('('));
        self.compile_expression();
        self.eat(&Token::Symbol(')'));
        self.eat(&Token::Symbol('{'));
        self.compile_statements();
        self.eat(&Token::Symbol('}'));

        // else clause
        if self.peek().is_some_and(|next_token| next_token == &Token::Keyword("else".to_string())) {
            self.eat(&Token::Keyword("else".to_string())); 
            self.eat(&Token::Symbol('{'));
            self.compile_statements();
            self.eat(&Token::Symbol('}'));
        }
    }

    fn compile_while(&mut self) {
        self.eat(&Token::Keyword("while".to_string())); 

        self.eat(&Token::Symbol('('));
        self.compile_expression();
        self.eat(&Token::Symbol(')'));

        self.eat(&Token::Symbol('{'));
        self.compile_statements();
        self.eat(&Token::Symbol('}'));
    }

    fn compile_do(&mut self) {
        self.eat(&Token::Keyword("do".to_string())); 
        self.eat_independent(&Token::Identifier(String::from("")));

        // if member subroutine call
        if self.peek().is_some_and(|next_token| next_token == &Token::Symbol('.')) {
            self.eat(&Token::Symbol('.'));
            self.eat_independent(&Token::Identifier(String::from("")));
        }

        self.eat(&Token::Symbol('('));
        self.compile_expression_list();
        self.eat(&Token::Symbol(')'));

        self.eat(&Token::Symbol(';'));
    }

    fn compile_return(&mut self) {
        self.eat(&Token::Keyword("return".to_string())); 

        if self.peek().is_some_and(|next_token| next_token != &Token::Symbol(';')) {
            self.compile_expression();
        }
        self.eat(&Token::Symbol(';'));
    }

    fn compile_expression(&mut self) {
    }

    fn compile_term(&mut self) {
    }

    fn compile_expression_list(&mut self) {
    }
}
