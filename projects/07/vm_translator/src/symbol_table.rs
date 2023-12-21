use std::{
    collections::HashMap,
    default,
};


pub struct SymbolTable {
    table: HashMap<String, String>,
    current_address: u32,
    max_address: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { 
            table: HashMap::new(),
            current_address: 0, 
            max_address: 0,
        }
    }

    pub fn set_symbol(&mut self, symbol: &str, value: &str) {
        self.table.insert(symbol.to_string(), value.to_string());
    }

    pub fn get_symbol(&mut self, symbol: &str) -> String {
        if self.table.contains_key(symbol) {
            return self.table.get(symbol).unwrap().to_string();
        }
        else if symbol.chars().all(|c| c.is_digit(10)) {
            return symbol.to_string();
        }
        else {
            self.current_address += 1;
            self.table.insert(symbol.to_string(), self.current_address.to_string());
            return self.current_address.to_string();
        }
    }


}

impl default::Default for SymbolTable{
    fn default() -> Self {
        let temp: HashMap<_, _> = HashMap::from(
        [
            ("R0", "0"),
            ("R1", "1"),
            ("R2", "2"),
            ("R3", "3"),
            ("R4", "4"),
            ("R5", "5"),
            ("R6", "6"),
            ("R7", "7"),
            ("R8", "8"),
            ("R9", "9"),
            ("R10", "10"),
            ("R11", "11"),
            ("R12", "12"),
            ("R13", "13"),
            ("R14", "14"),
            ("R15", "15"),
            ("SP", "0"),
            ("LCL", "1"),
            ("ARG", "2"),
            ("THIS", "3"),
            ("THAT", "4"),
            ("SCREEN", "16384"),
            ("KBD", "24576"),
        ]);

        let mut table: HashMap<String, String> = HashMap::new();
        for entry in temp.iter() {
            table.insert(entry.0.to_string(), entry.1.to_string());
        }

        let mut symbol_table = SymbolTable { 
            table,
            current_address: 15, 
            max_address: 24576,
        };


        symbol_table
    }
}


