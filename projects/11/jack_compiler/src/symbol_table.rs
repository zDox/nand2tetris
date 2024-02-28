use std::{ 
    collections::HashMap,
    default::Default,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum SymbolKind{
    STATIC,
    FIELD,
    ARG,
    VAR
}

pub struct Symbol {
    pub name: String,
    pub symbol_type: String,
    pub kind: SymbolKind,
    pub index: u32,
}

impl Default for Symbol{
    fn default() -> Self {
        Self{
            name: "".to_string(),
            symbol_type: "".to_string(),
            kind: SymbolKind::ARG,
            index: 0,
        }
    }
}

pub struct SymbolTable {
    table: HashMap<String, Symbol>,
    table_name: String,
    var_count_map: HashMap<SymbolKind, u32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            var_count_map: HashMap::new(),
            table_name: String::from(""),
        }
    }

    pub fn reset(&mut self, table_name: &str) {
        self.table.clear();
        self.var_count_map.clear();
        self.table_name = table_name.to_string();
    }

    pub fn get_table_name(&self) -> &str {
        &self.table_name
    }

    pub fn define(&mut self, name: &str, symbol_type: &str, kind: &SymbolKind) {
        let var_count: &mut u32 = self.var_count_map.entry(*kind).or_insert(0);
        *var_count += 1;

        self.table.insert(name.to_string(), 
                          Symbol{
                              name: name.to_string(), 
                              symbol_type: symbol_type.to_string(), 
                              kind: *kind,
                              index: *var_count-1,
                          });
    }

    pub fn var_count(&mut self, kind: &SymbolKind) -> u32 {
        *self.var_count_map.entry(*kind).or_insert(0)
    }

    pub fn kind_of(&self, name: &str) -> Option<SymbolKind> {
        self.table.get(name).and_then(|val| Some(val.kind))
    }

    pub fn type_of(&self, name: &str) -> Option<&str> {
        self.table.get(name).and_then(|val| Some(val.symbol_type.as_str()))
    }

    pub fn index_of(&self, name: &str) -> Option<u32> {
        self.table.get(name).and_then(|val| Some(val.index))
    }
}
