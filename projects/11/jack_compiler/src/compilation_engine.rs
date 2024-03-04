use std::path::Path;
use super::tokenizer::Token;
use super::symbol_table::{ SymbolTable, SymbolKind };

#[path = "vm_writer.rs"]
mod vm_writer;
use vm_writer::{VMWriter, Segment, ArithmeticCommand};


pub struct CompilationEngine {
    index: usize,
    label_counter: usize,
    tokens: Vec<Token>,
    writer: VMWriter,
    class_scope: SymbolTable,
    function_scope: SymbolTable,
}

impl CompilationEngine {
    pub fn new(tokens: Vec<Token>, output_file: &Path) -> Self {
        Self { 
            index: 0,
            label_counter: 0,
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

    fn increase_label_counter(&mut self) -> usize {
        self.label_counter += 1;
        self.label_counter
    }

    fn eat(&mut self, to_eat_token: &Token) {
        let next = self.next();
        if to_eat_token == next {
            println!("Eat: {}", next);
        }
        else {
            panic!("Next token {} is not expected. Expected '{}'.", next, to_eat_token);
        }
    }


    fn eat_independent(&mut self, to_eat_token: &Token) -> Token{
        let next_token = self.peek();
        if next_token.equals_type(to_eat_token){
            let copy = next_token.clone();
            self.eat(&copy);
            return copy.clone();
        }
        panic!("Next token {} is not expected. Expected '{}'.", next_token, to_eat_token);
    }


    fn eat_tokens(&mut self, to_eat_tokens: &Vec<Token>) -> Token{
        let next_token = self.peek();
        if to_eat_tokens.contains(next_token){
            let copy = next_token.clone();
            self.eat(&copy);
            return copy.clone();
        }
        panic!("Next token '{}' is not expected.", next_token);
    }

    fn eat_identifier(&mut self) -> String {
        match self.eat_independent(&Token::Identifier("".to_string())) {
            Token::Identifier(indentifier) => indentifier.clone(),
            _ => unreachable!(),
        }
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
        self.eat(&Token::Keyword("class".to_string()));
        let class_name = match self.eat_independent(&Token::Identifier(String::from(""))) {
            Token::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };
        self.eat(&Token::Symbol('{'));


        self.class_scope.reset(&class_name);

        while [Token::Keyword("static".to_string()), Token::Keyword("field".to_string())].contains(self.peek()) {
            self.compile_class_var_dec();
        }

        while [Token::Keyword("function".to_string()), Token::Keyword("method".to_string()), Token::Keyword("constructor".to_string())].contains(self.peek()) {
            self.compile_subroutine();
        }

        self.eat(&Token::Symbol('}'));

    }

    fn compile_class_var_dec(&mut self) {
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
    }

    fn compile_subroutine(&mut self) {
        let subroutine_type = self.eat_tokens(&vec!(Token::Keyword("function".to_string()), Token::Keyword("method".to_string()), Token::Keyword("constructor".to_string())));

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
        
        self.function_scope.reset(&name);

        if subroutine_type == Token::Keyword("method".to_string()) {
            self.function_scope.define("this", "className", &SymbolKind::ARG);
        }


        self.eat(&Token::Symbol('('));
        self.compile_parameter_list();
        self.eat(&Token::Symbol(')'));

        self.eat(&Token::Symbol('{'));

        if self.peek() == &Token::Keyword("var".to_string()) {
            while self.peek() == &Token::Keyword("var".to_string()) {
                self.compile_var_dec();
            }
        }

        self.writer.write_function(&format!("{}.{}", self.class_scope.get_table_name(), &name),
                                   self.function_scope.var_count(&SymbolKind::VAR));

        // Method
        if subroutine_type == Token::Keyword("method".to_string()) {
            self.writer.write_push(&Segment::ARGUMENT, 0);
            self.writer.write_pop(&Segment::POINTER, 0);
        }

        // Constructor
        else if subroutine_type == Token::Keyword("constructor".to_string()) {
            self.writer.write_push(&Segment::CONSTANT, self.class_scope.var_count(&SymbolKind::FIELD));
            self.writer.write_call("Memory.alloc", 1);
            self.writer.write_pop(&Segment::POINTER, 0);
        }


        self.compile_statements();

        self.eat(&Token::Symbol('}'));

    }

    fn compile_parameter_list(&mut self) {
        while [Token::Keyword("boolean".to_string()), Token::Keyword("int".to_string()), Token::Keyword("char".to_string())].contains(self.peek()) {
            let var_type;
            if self.peek().equals_type(&Token::Identifier("".to_string())) {
                var_type = match self.eat_independent(&Token::Identifier(String::from(""))) {
                    Token::Identifier(identifier) => identifier,
                    _ => unreachable!(),
                };
            }
            else {
                var_type = match self.eat_tokens(&vec!(Token::Keyword("boolean".to_string()), Token::Keyword("int".to_string()), Token::Keyword("char".to_string()))) {
                    Token::Keyword(keyword) => keyword,
                    _ => unreachable!(),
                }; 
            }

            let var_name = match self.eat_independent(&Token::Identifier(String::from(""))) {
                Token::Identifier(indentifier) => indentifier,
                _ => unreachable!(),
            };

            self.function_scope.define(&var_name, &var_type, &SymbolKind::ARG);
            println!("Parameter: {} {} arg: {}", var_name, var_type, self.function_scope.index_of(&var_name).unwrap());

            if self.peek() == &Token::Symbol(','){
                self.eat(&Token::Symbol(','));
            }
            else {
                continue;
            }
        }
    }

    fn compile_var_dec(&mut self) {
        self.eat(&Token::Keyword("var".to_string())); 

        let var_type;
        if self.peek().equals_type(&Token::Identifier("".to_string())) {
            var_type = match self.eat_independent(&Token::Identifier(String::from(""))) {
                Token::Identifier(identifier) => identifier,
                _ => unreachable!(),
            };
        }
        else {
            var_type = match self.eat_tokens(&vec!(Token::Keyword("boolean".to_string()), Token::Keyword("int".to_string()), Token::Keyword("char".to_string()))) {
                Token::Keyword(keyword) => keyword,
                _ => unreachable!(),
            }; 
        }

        let mut var_name = match self.eat_independent(&Token::Identifier(String::from(""))) {
            Token::Identifier(indentifier) => indentifier,
            _ => unreachable!(),
        };

        self.function_scope.define(&var_name, &var_type, &SymbolKind::VAR);
        println!("Var: {}, {}", var_name, var_type);

        while self.peek() == &Token::Symbol(',') {
            self.eat(&Token::Symbol(','));
            var_name = match self.eat_independent(&Token::Identifier(String::from(""))) {
                Token::Identifier(indentifier) => indentifier,
                _ => unreachable!(),
            };
            self.function_scope.define(&var_name, &var_type, &SymbolKind::VAR);
            println!("Var: {}, {}", var_name, var_type);
        }

        self.eat(&Token::Symbol(';'));

    }

    fn compile_statements(&mut self) {
        while self.peek().equals_type(&Token::Keyword("".to_string())) {
            let next_token = self.peek();
            if let Token::Keyword(keyword) = next_token{
                match keyword.as_str(){
                    "let" => self.compile_let(),
                    "if" => self.compile_if(),
                    "while" => self.compile_while(),
                    "do" => self.compile_do(),
                    "return" => self.compile_return(),
                    _ => unreachable!(),
                }
            }
            else {
                unreachable!();
            }
        }

    }

    fn compile_let(&mut self) {
        self.eat(&Token::Keyword("let".to_string())); 
        let name = match self.eat_independent(&Token::Identifier(String::from(""))) {
            Token::Identifier(identifier) => identifier,
            _ => unreachable!()
        };

        if self.peek() == &Token::Symbol('[') {
            self.eat(&Token::Symbol('['));
            self.compile_expression();
            self.eat(&Token::Symbol(']'));
        }

        self.eat(&Token::Symbol('='));
        self.compile_expression();
        self.eat(&Token::Symbol(';'));
        
        
        let kind: SymbolKind;
        let index: u32;
        if let Some(symbol_kind) = self.function_scope.kind_of(&name) {
            kind = symbol_kind;
            index = self.function_scope.index_of(&name).expect("Expected index");
        }
        else if let Some(symbol_kind) = self.class_scope.kind_of(&name) {
            kind = symbol_kind;
            index = self.class_scope.index_of(&name).expect("Expected index");
        }
        else {
            panic!("Expected to find symbol with name '{}' in symbol table", name);
        }
        let segment = match kind {
            SymbolKind::STATIC => Segment::STATIC,
            SymbolKind::ARG => Segment::ARGUMENT,
            SymbolKind::VAR => Segment::LOCAL,
            SymbolKind::FIELD => Segment::THIS,
        };

        self.writer.write_pop(&segment, index);
    }

    fn compile_if(&mut self) {
        let label_1 = &format!("IF_TRUE_{}", self.label_counter);
        let label_2 = &format!("IF_WHILE_{}", self.increase_label_counter());

        self.eat(&Token::Keyword("if".to_string())); 
        self.eat(&Token::Symbol('('));
        self.compile_expression();
        self.writer.write_arithmetic(&ArithmeticCommand::NOT);
        self.writer.write_if(label_1);

        self.eat(&Token::Symbol(')'));
        self.eat(&Token::Symbol('{'));
        self.compile_statements();
        self.eat(&Token::Symbol('}'));
        
        self.writer.write_goto(label_2);
        self.writer.write_label(label_1);

        // else clause
        if self.peek() == &Token::Keyword("else".to_string()) {
            self.eat(&Token::Keyword("else".to_string())); 
            self.eat(&Token::Symbol('{'));
            self.compile_statements();
            self.eat(&Token::Symbol('}'));
        }

        self.writer.write_label(label_2);
    }

    fn compile_while(&mut self) {
        let label_1 = &format!("WHILE_TRUE_{}", self.label_counter);
        let label_2 = &format!("WHILE_FALSE_{}", self.increase_label_counter());

        self.eat(&Token::Keyword("while".to_string())); 


        self.writer.write_label(label_1);

        self.eat(&Token::Symbol('('));
        self.compile_expression();
        self.eat(&Token::Symbol(')'));

        self.writer.write_arithmetic(&ArithmeticCommand::NOT);
        self.writer.write_if(label_2);

        self.eat(&Token::Symbol('{'));
        self.compile_statements();
        self.eat(&Token::Symbol('}'));

        self.writer.write_goto(label_1);
        self.writer.write_label(label_2);
    }

    fn compile_do(&mut self) {
        self.eat(&Token::Keyword("do".to_string()));

        self.compile_expression();
        self.writer.write_pop(&Segment::TEMP, 0);
        self.eat(&Token::Symbol(';'));
    }

    fn compile_return(&mut self) {
        self.eat(&Token::Keyword("return".to_string())); 

        if self.peek() != &Token::Symbol(';') {
            self.compile_expression();
        }
        self.eat(&Token::Symbol(';'));
        self.writer.write_return();
    }

    fn compile_expression(&mut self) {

        self.compile_term();
        while [
            Token::Symbol('+'), Token::Symbol('-'), Token::Symbol('*'), Token::Symbol('/'), Token::Symbol('&'),
            Token::Symbol('|'), Token::Symbol('<'), Token::Symbol('>'), Token::Symbol('=')]
                .contains(self.peek()) {
            let operator = match self.eat_tokens(&vec!(
                    Token::Symbol('+'), Token::Symbol('-'), Token::Symbol('*'), Token::Symbol('/'), 
                    Token::Symbol('&'), Token::Symbol('|'), Token::Symbol('<'), Token::Symbol('>'),
                    Token::Symbol('='))) {
                Token::Symbol(symbol) => symbol,
                _ => unreachable!(),
            };
            self.compile_term();

            match operator {
                '*' => {
                    self.writer.write_call("Math.multiply", 2);
                    return;
                }
                '/' => {
                    self.writer.write_call("Math.divide", 2);
                    return;
                }
                 _  => (),
            }
            
            let arithmetic_command = match operator {
                '+' => ArithmeticCommand::ADD,
                '-' => ArithmeticCommand::SUB,
                '&' => ArithmeticCommand::AND,
                '|' => ArithmeticCommand::OR,
                '>' => ArithmeticCommand::GT,
                '<' => ArithmeticCommand::LT,
                '=' => ArithmeticCommand::EQ,
                 _  => unreachable!(),
            };
            self.writer.write_arithmetic(&arithmetic_command);
        }

    }

    fn compile_term(&mut self) {
        let next_token: Token = self.peek().clone();
        match next_token {
            Token::Keyword(keyword) => {
                self.eat(&Token::Keyword(keyword.to_string()));
                match keyword.as_str(){
                    "true" => {
                        self.writer.write_push(&Segment::CONSTANT, 0);
                        self.writer.write_arithmetic(&ArithmeticCommand::NOT);
                    },
                    "false" | "null" => {
                        self.writer.write_push(&Segment::CONSTANT, 0);
                    },
                    "this" => {
                        self.writer.write_push(&Segment::POINTER, 0);
                    }
                    _ => unreachable!(),
                }
            }
            Token::Symbol('(') => {
                self.eat(&Token::Symbol('('));
                self.compile_expression();
                self.eat(&Token::Symbol(')'));
            }
            next_token if [Token::Symbol('-'), Token::Symbol('~')].contains(&next_token) => {
                self.eat(&next_token);
                let arithmetic_command = match next_token {
                    Token::Symbol('~') => ArithmeticCommand::NOT,
                    Token::Symbol('-') => ArithmeticCommand::NEG,
                    _ => unreachable!(),
                };
                self.compile_term();
                self.writer.write_arithmetic(&arithmetic_command);
            }
            Token::IntegerConstant(integer) => {
                self.eat(&Token::IntegerConstant(integer));
                self.writer.write_push(&Segment::CONSTANT, integer);
            }
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
                    // call subroutine
                    Token::Symbol('(') | Token::Symbol('.') => {
                        let function_name: String;
                        let mut class_name;
                        if look_ahead_token == Token::Symbol('(') {
                            function_name = self.eat_identifier();
                            class_name = "this".to_string();
                        } 
                        else {
                            class_name = self.eat_identifier();
                            self.eat(&Token::Symbol('.'));
                            function_name = self.eat_identifier();
                        }

                        self.eat(&Token::Symbol('('));
                        
                        let mut arguments = 0;

                        if class_name == "this" {
                            self.writer.write_push(&Segment::POINTER, 0);
                            class_name = self.class_scope.get_table_name().to_string();
                            arguments += 1;
                        }
                        else if self.function_scope.has_entry(&class_name) {
                            let segment = match self.function_scope.kind_of(&class_name).unwrap() {
                                SymbolKind::ARG => Segment::ARGUMENT,
                                SymbolKind::VAR => Segment::LOCAL,
                                SymbolKind::FIELD => Segment::THIS,
                                SymbolKind::STATIC => Segment::STATIC,
                            };
                            self.writer.write_push(&segment, self.function_scope.index_of(&class_name).unwrap());
                            class_name = self.function_scope.type_of(&class_name).unwrap().to_string();
                            arguments += 1;
                        }

                        else if self.class_scope.has_entry(&class_name) {
                            let segment = match self.class_scope.kind_of(&class_name).unwrap() {
                                SymbolKind::ARG => Segment::ARGUMENT,
                                SymbolKind::VAR => Segment::LOCAL,
                                SymbolKind::FIELD => Segment::THIS,
                                SymbolKind::STATIC => Segment::STATIC,
                            };
                            self.writer.write_push(&segment, self.class_scope.index_of(&class_name).unwrap());
                            class_name = self.class_scope.type_of(&class_name).unwrap().to_string();
                            arguments += 1;
                        }


                        arguments += self.compile_expression_list();

                        self.writer.write_call(&format!("{}.{}", class_name, function_name), arguments);

                        self.eat(&Token::Symbol(')'));
                    },
                    
                    Token::None => unreachable!(),

                    // Else should be a variable
                    _ => {
                        self.eat(&Token::Identifier(identifier.clone()));
                        let segment;
                        let index;
                        println!("Variable: {}", identifier);

                        if self.function_scope.has_entry(&identifier) {
                            segment = match self.function_scope.kind_of(&identifier).unwrap() {
                                SymbolKind::ARG => Segment::ARGUMENT,
                                SymbolKind::VAR => Segment::LOCAL,
                                SymbolKind::FIELD => Segment::THIS,
                                SymbolKind::STATIC => Segment::STATIC,
                            };
                            index = self.function_scope.index_of(&identifier).unwrap();
                        }

                        else {
                            segment = match self.class_scope.kind_of(&identifier).unwrap() {
                                SymbolKind::ARG => Segment::ARGUMENT,
                                SymbolKind::VAR => Segment::LOCAL,
                                SymbolKind::FIELD => Segment::THIS,
                                SymbolKind::STATIC => Segment::STATIC,
                            };
                            index = self.class_scope.index_of(&identifier).unwrap();
                        }

                        self.writer.write_push(&segment, index);
                    }
                }
            },
            _ => (),
        }
    }

    fn compile_expression_list(&mut self) -> u32 {
        let mut counter = 0; 

        while self.peek() != &Token::Symbol(')') {
            self.compile_expression();
            counter += 1;

            if self.peek() == &Token::Symbol(',') {
                self.eat(&Token::Symbol(','));
                continue;
            }
            else {
                return counter;
            }
        }
        return counter
    }
}
