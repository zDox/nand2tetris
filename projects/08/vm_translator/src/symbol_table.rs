use std::collections::HashMap;


pub struct SymbolTable {
    table: HashMap<String, String>,
    current_address: u32,
}

impl SymbolTable {
    pub fn new(current_address: u32) -> Self {
        SymbolTable { 
            table: HashMap::new(),
            current_address, 
        }
    }

    pub fn set_symbol(&mut self, symbol: &str, value: &str) {
        self.table.insert(symbol.to_string(), value.to_string());
    }

    pub fn get_symbol(&mut self, symbol: &str) -> String {
        if self.table.contains_key(symbol) {
            return self.table.get(symbol).unwrap().to_string();
        }
        else {
            self.current_address += 1;
            self.table.insert(symbol.to_string(), self.current_address.to_string());
            return self.current_address.to_string();
        }
    }


}
