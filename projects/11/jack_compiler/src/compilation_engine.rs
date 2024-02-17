use std::path::Path;
use super::tokenizer::Token;
use super::symbol_table::{ SymbolTable, SymbolKind };

#[path = "vm_writer.rs"]
mod vm_writer;
use vm_writer::VMWriter;


pub struct CompilationEngine {
    indent: usize,
    index: usize,
    tokens: Vec<Token>,
    writer: VMWriter,
    class_scope: SymbolTable,
    function_scope: SymbolTable,
}

impl CompilationEngine {
    pub fn new(tokens: Vec<Token>, output_file: &Path) -> Self {
        Self { 
            indent: 0,
            index: 0,
            tokens,
            writer: VMWriter::new(output_file),
            class_scope: SymbolTable::new(),
            function_scope: SymbolTable::new(),
        }
    }

    pub fn run(&mut self) {
        self.compile_class();
        self.writer.close();
    }

    fn write_line(&mut self, line: &str) {
        let line_str = &(" ".repeat(self.indent*2) + &line + "\n");
        self.writer.write(line_str);
        println!("{}", line_str);
    }

    fn write_xml(&mut self, tag: &str, content: &str) {
        self.write_line(&format!("<{}> {} </{}>", tag, content, tag));
    }

    fn write_tag(&mut self, tag: &str) {
        self.write_line(&format!("<{}>", tag));
        self.indent += 1;
    }

    fn write_end_tag(&mut self, tag: &str) {
        self.indent -= 1;
        self.write_line(&format!("</{}>", tag));
    }

    fn eat(&mut self, to_eat_token: &Token) {
        let next = self.next();
        if to_eat_token == next {
            let copy = next.clone();
            self.write_line(&(copy.to_xml()));
        }
        else {
            panic!("Next token None is not expected. Expected '{}'.", to_eat_token);
        }
    }


    fn eat_independent(&mut self, to_eat_token: &Token) -> Token{
        let next_token = self.next();
        if next_token.equals_type(to_eat_token){
            let copy = next_token.clone();
            self.eat(&copy);
            return copy.clone();
        }
        panic!("Next token None is not expected. Expected '{}'.", to_eat_token);
    }


    fn eat_tokens(&mut self, to_eat_tokens: &Vec<Token>) -> Token{
        let next_token = self.next();
        if to_eat_tokens.contains(next_token){
            let copy = next_token.clone();
            self.eat(&copy);
            return copy.clone();
        }
        panic!("Next token '{}' is not expected.", next_token);
    }

    fn peek(& self) -> &Token {
        self.tokens.get(self.index).expect("No next token")
    }

    fn next(&mut self) -> &Token {
        let next = self.tokens.get(self.index);
        self.index += 1;
        next.expect("No next token")
    }


    fn compile_class(&mut self) {
        self.write_tag("class");

        self.class_scope.reset();

        self.eat(&Token::Keyword("class".to_string()));
        self.eat_independent(&Token::Identifier(String::from("")));
        self.eat(&Token::Symbol('{'));

        while [Token::Keyword("static".to_string()), Token::Keyword("field".to_string())].contains(self.peek()) {
            self.compile_class_var_dec();
        }

        while [Token::Keyword("function".to_string()), Token::Keyword("method".to_string()), Token::Keyword("constructor".to_string())].contains(self.peek()) {
            self.compile_subroutine();
        }

        self.eat(&Token::Symbol('}'));

        self.write_end_tag("class");
    }

    fn compile_class_var_dec(&mut self) {
        self.write_tag("classVarDec");

        let symbol_kind: SymbolKind = match self.eat_tokens(&vec!(Token::Keyword("field".to_string()), Token::Keyword("static".to_string()))) {
            Token::Keyword(ref keyword) if keyword == "field" => SymbolKind::FIELD,
            Token::Keyword(ref keyword) if keyword == "static" => SymbolKind::STATIC,
            _ => unreachable!()
        }; 
        

        let symbol_type: String;
        if self.peek().equals_type(&Token::Identifier("".to_string())) {
            symbol_type = match self.eat_independent(&Token::Identifier(String::from(""))) {
                Token::Identifier(indentifier) => indentifier,
                _ => unreachable!()
            };
        }
        else {
            symbol_type = match self.eat_tokens(&vec!(Token::Keyword("boolean".to_string()), Token::Keyword("int".to_string()), Token::Keyword("char".to_string()))) {
                Token::Keyword(keyword) => keyword,
                _ => unreachable!(),
            }; 
        }

        let mut symbol_name = match self.eat_independent(&Token::Identifier(String::from(""))){
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };

        self.class_scope.define(&symbol_name, &symbol_type, &symbol_kind);

        while self.peek() == &Token::Symbol(',') {
            self.eat(&Token::Symbol(','));
            symbol_name = match self.eat_independent(&Token::Identifier(String::from(""))) {
                Token::Identifier(identifier) => identifier,
                _ => unreachable!()
            };
            self.class_scope.define(&symbol_name, &symbol_type, &symbol_kind);
        }
        self.eat(&Token::Symbol(';'));
        
        self.write_end_tag("classVarDec");
    }

    fn compile_subroutine(&mut self) {
        self.write_tag("subroutineDec");

        self.function_scope.reset();

        let subroutine_type = self.eat_tokens(&vec!(Token::Keyword("function".to_string()), Token::Keyword("method".to_string()), Token::Keyword("constructor".to_string())));

        if subroutine_type == Token::Keyword("method".to_string()) {
            self.function_scope.define("this", "className", &SymbolKind::ARG);
        }

        // if type consume it else its an identifier for a class type
        if [Token::Keyword("void".to_string()),
            Token::Keyword("boolean".to_string()), Token::Keyword("int".to_string()), Token::Keyword("char".to_string())].contains(self.peek()) {

            self.eat_tokens(&vec!(Token::Keyword("void".to_string()), 
                                  Token::Keyword("boolean".to_string()), Token::Keyword("int".to_string()), Token::Keyword("char".to_string())));
        }
        else {
            self.eat_independent(&Token::Identifier(String::from("")));
        }
        

        let name = match self.eat_independent(&Token::Identifier(String::from(""))) {
            Token::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };

        self.eat(&Token::Symbol('('));
        let args =self.compile_parameter_list();
        self.eat(&Token::Symbol(')'));

        self.compile_subroutine_body();

        self.write_end_tag("subroutineDec");
    }

    fn compile_parameter_list(&mut self) {
        self.write_tag("parameterList");


        while [Token::Keyword("boolean".to_string()), Token::Keyword("int".to_string()), Token::Keyword("char".to_string())].contains(self.peek()) {

            self.eat_tokens(&vec!(Token::Keyword("boolean".to_string()), Token::Keyword("int".to_string()), Token::Keyword("char".to_string()))); 
            self.eat_independent(&Token::Identifier(String::from("")));

            if self.peek() == &Token::Symbol(','){
                self.eat(&Token::Symbol(','));
            }
            else {
                continue;
            }
        }

        self.write_end_tag("parameterList");
    }

    fn compile_subroutine_body(&mut self) {
        self.write_tag("subroutineBody");

        self.eat(&Token::Symbol('{'));
        
        while self.peek() == &Token::Keyword("var".to_string()) {
            self.compile_var_dec();
        }

        self.compile_statements();

        self.eat(&Token::Symbol('}'));

        self.write_end_tag("subroutineBody");
    }

    fn compile_var_dec(&mut self) {
        self.write_tag("varDec");

        self.eat(&Token::Keyword("var".to_string())); 

        if self.peek().equals_type(&Token::Identifier("".to_string())) {
            self.eat_independent(&Token::Identifier(String::from("")));
        }
        else {
            self.eat_tokens(&vec!(Token::Keyword("boolean".to_string()), Token::Keyword("int".to_string()), Token::Keyword("char".to_string()))); 
        }

        self.eat_independent(&Token::Identifier(String::from("")));

        while self.peek() == &Token::Symbol(',') {
            self.eat(&Token::Symbol(','));
            self.eat_independent(&Token::Identifier(String::from("")));
        }

        self.eat(&Token::Symbol(';'));

        self.write_end_tag("varDec");
    }

    fn compile_statements(&mut self) {
        self.write_tag("statements");

        while self.peek().equals_type(&Token::Keyword("".to_string())) {
            let next_token = self.peek();
            if let Token::Keyword(keyword) = next_token{
                match keyword.as_str(){
                    "let" => self.compile_let(),
                    "if" => self.compile_if(),
                    "while" => self.compile_while(),
                    "do" => self.compile_do(Some(true)),
                    "return" => self.compile_return(),
                    _ => (),
                }
            }
        }

        self.write_end_tag("statements");
    }

    fn compile_let(&mut self) {
        self.write_tag("letStatement");

        self.eat(&Token::Keyword("let".to_string())); 
        self.eat_independent(&Token::Identifier(String::from("")));

        if self.peek() == &Token::Symbol('[') {
            self.eat(&Token::Symbol('['));
            self.compile_expression();
            self.eat(&Token::Symbol(']'));
        }

        self.eat(&Token::Symbol('='));
        self.compile_expression();
        self.eat(&Token::Symbol(';'));

        self.write_end_tag("letStatement");
    }

    fn compile_if(&mut self) {
        self.write_tag("ifStatement");

        self.eat(&Token::Keyword("if".to_string())); 
        self.eat(&Token::Symbol('('));
        self.compile_expression();
        self.eat(&Token::Symbol(')'));
        self.eat(&Token::Symbol('{'));
        self.compile_statements();
        self.eat(&Token::Symbol('}'));

        // else clause
        if self.peek() == &Token::Keyword("else".to_string()) {
            self.eat(&Token::Keyword("else".to_string())); 
            self.eat(&Token::Symbol('{'));
            self.compile_statements();
            self.eat(&Token::Symbol('}'));
        }
        self.write_end_tag("ifStatement");
    }

    fn compile_while(&mut self) {
        self.write_tag("whileStatement");

        self.eat(&Token::Keyword("while".to_string())); 

        self.eat(&Token::Symbol('('));
        self.compile_expression();
        self.eat(&Token::Symbol(')'));

        self.eat(&Token::Symbol('{'));
        self.compile_statements();
        self.eat(&Token::Symbol('}'));

        self.write_end_tag("whileStatement");
    }

    fn compile_do(&mut self, with_do: Option<bool>) {

        if with_do.is_some_and(|val| val) {
            self.write_tag("doStatement");
            self.eat(&Token::Keyword("do".to_string())); 
        }
        self.eat_independent(&Token::Identifier(String::from("")));

        // if member subroutine call
        if self.peek() == &Token::Symbol('.') {
            self.eat(&Token::Symbol('.'));
            self.eat_independent(&Token::Identifier(String::from("")));
        }

        self.eat(&Token::Symbol('('));
        self.compile_expression_list();
        self.eat(&Token::Symbol(')'));

        if with_do.is_some_and(|val| val) {
            self.eat(&Token::Symbol(';'));
            self.write_end_tag("doStatement");
        }
    }

    fn compile_return(&mut self) {
        self.write_tag("returnStatement");

        self.eat(&Token::Keyword("return".to_string())); 

        if self.peek() != &Token::Symbol(';') {
            self.compile_expression();
        }
        self.eat(&Token::Symbol(';'));

        self.write_end_tag("returnStatement");
    }

    fn compile_expression(&mut self) {
        self.write_tag("expression");

        self.compile_term();
        while [
            Token::Symbol('+'), Token::Symbol('-'), Token::Symbol('*'), Token::Symbol('/'), Token::Symbol('&'),
            Token::Symbol('|'), Token::Symbol('<'), Token::Symbol('>'), Token::Symbol('=')]
                .contains(self.peek()) {
            self.eat_tokens(&vec!(
                    Token::Symbol('+'), Token::Symbol('-'), Token::Symbol('*'), Token::Symbol('/'), 
                    Token::Symbol('&'), Token::Symbol('|'), Token::Symbol('<'), Token::Symbol('>'),
                    Token::Symbol('=')));
            self.compile_term();
        }

        self.write_end_tag("expression");
    }

    fn compile_term(&mut self) {
        self.write_tag("term");

        let next_token: Token = self.peek().clone();
        match next_token {
            Token::Keyword(keyword) => self.eat(&Token::Keyword(keyword.to_string())),
            Token::Symbol('(') => {
                self.eat(&Token::Symbol('('));
                self.compile_expression();
                self.eat(&Token::Symbol(')'));
            }
            next_token if [Token::Symbol('-'), Token::Symbol('~')].contains(&next_token) => {
                self.eat(&next_token);
                self.compile_term();
            }
            Token::IntegerConstant(integer) => self.eat(&Token::IntegerConstant(integer)),
            Token::StringConstant(string) => self.eat(&Token::StringConstant(string)),
            Token::Identifier(identifier) => {
                self.index += 1;
                let look_ahead_token = self.peek().clone();
                self.index -= 1;
                match look_ahead_token {
                    // Array access
                    Token::Symbol('[') => {
                        self.eat(&Token::Identifier(identifier));
                        self.eat(&Token::Symbol('['));
                        self.compile_expression();
                        self.eat(&Token::Symbol(']'));
                    }
                    // subroutine call
                    Token::Symbol('(') | Token::Symbol('.') => self.compile_do(None),
                    
                    Token::None => unreachable!(),

                    // Else should be a variable
                    _ => self.eat(&Token::Identifier(identifier)),
                }
            },
            _ => (),
        }
        self.write_end_tag("term");
    }

    fn compile_expression_list(&mut self) -> u32 {
        self.write_tag("expressionList");
        let mut counter = 1; 

        self.compile_expression();
        while self.peek() == &Token::Symbol(',') {
            self.eat(&Token::Symbol(','));
            self.compile_expression();
            counter += 1;
        }
        self.write_end_tag("expressionList");
        return counter
    }
}
