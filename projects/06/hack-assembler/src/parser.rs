use std::{
    fs::read_to_string,
    path::Path,
    fmt,
};

#[path = "symbol_table.rs"]
mod symbol_table;
use symbol_table::SymbolTable;


pub enum Instruction {
    A(String),
    L(String),
    C(String, String, String),
    None,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
            Instruction::A(s) => write!(f, "A_Inst: {}", s),
            Instruction::L(s) => write!(f, "L_Inst: {}", s),
            Instruction::C(dest, comp, jump) => write!(f, "C_Inst: D: {}, C: {}, J: {}", dest, comp, jump),
            Instruction::None => write!(f, "None"),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    FileType,
    InvalidInstruction,
}


pub struct Parser {
    symbol_table: SymbolTable,
}

impl Parser {
    pub fn new() -> Self {
        Parser{symbol_table: SymbolTable::default()}
    }
    pub fn parse(&mut self, path: &Path) -> Result<String, ParseError> {

        // return Err if extension is not asm
        if !path.extension().is_some_and(|value| value == "asm") {
            return Err(ParseError::FileType);
        }

        let content = read_to_string(path).unwrap();
        let instructions = self.pass1(content)?;
        self.pass2(instructions)
    }

    fn parse_line(line: &str) -> Result<Instruction, ParseError> {
        let mut instruction: Result<Instruction, ParseError> = Ok(Instruction::None);
        println!("line: {}", line);

        if line.starts_with("//") || line.is_empty(){
            return instruction;
        }
        instruction = Self::parse_a_inst(line);
        if instruction.is_ok() {
            return Ok(instruction.unwrap());
        }

        instruction = Self::parse_l_inst(line);
        if instruction.is_ok() {
            return Ok(instruction.unwrap());
        }

        instruction = Self::parse_c_inst(line);
        if instruction.is_ok() {
            return Ok(instruction.unwrap());
        }
        Err(ParseError::InvalidInstruction)
    }

    fn pass1(&mut self, content: String) -> Result<Vec<Instruction>, ParseError> {
        let mut instructions: Vec<_> = vec!();
        let mut line_number: u32 = 0;

        for line in content.split("\n") {
            let mut instruction = Self::parse_line(line.trim()).unwrap();

            match instruction {
                Instruction::L(label_str) =>  {
                    instruction = Instruction::None;
                    println!("L: {} at {}", label_str, line_number);
                    self.symbol_table.set_symbol(&label_str, &line_number.to_string());
                },
                Instruction::A(_) | Instruction::C(_, _, _) => line_number += 1,
                _ => (),
            }

            println!("{}", instruction);
            match instruction {
                Instruction::None => (),
                _ => instructions.push(instruction),
            }
        }

        Ok(instructions)
    }

    fn pass2(&mut self, instructions: Vec<Instruction>) -> Result<String, ParseError> {
        let mut binary_str: String = String::from("");
        for mut instruction in instructions {

            match instruction {
                Instruction::A(symbol_str) => { 
                    instruction = Instruction::A(self.symbol_table.get_symbol(&symbol_str));
                }
                _ => (),
            }
            let instruction_code = self.generate_code(&instruction);
            binary_str.push_str(&((instruction_code) + &"\n"));
        }
        Ok(binary_str)
    }

    fn generate_code(&self, instruction: &Instruction) -> String {
        let mut line: String = String::from("");
        match instruction{
            Instruction::A(symbol_str) => {
                let addr: u32 = symbol_str.parse().unwrap();
                // Ensure the number fits within 15 bits
                let truncated_number = addr & 0b111_1111_1111_1111;

                // Convert the number to a 15-bit binary string
                let binary_string = format!("{:015b}", truncated_number);
                line = "0".to_string() + &binary_string;
            },
            Instruction::C(dest, comp, jump) => {
                line = self.generate_c_inst(dest, comp, jump);
            },
            _ => (),
        }

        line
    }

    fn generate_c_inst(&self, dest: &str, comp: &str, jump: &str) -> String {
        let comp_bits = match comp {
            "0"         => "0101010",
            "1"         => "0111111",
            "-1"        => "0111010",
            "D"         => "0001100",
            "A"         => "0110000",
            "M"         => "1110000",
            "!D"        => "0001101",
            "!A"        => "0110001",
            "!M"        => "1110001",
            "-D"        => "0001111",
            "-A"        => "0110011",
            "-M"        => "1110011",
            "D+1"       => "0011111",
            "A+1"       => "0110111",
            "M+1"       => "1110111",
            "D-1"       => "0001110",
            "A-1"       => "0110010",
            "M-1"       => "1110010",
            "D+A"       => "0000010",
            "D+M"       => "1000010",
            "D-A"       => "0010011",
            "D-M"       => "1010011",
            "A-D"       => "0000111",
            "M-D"       => "1000111",
            "D&A"       => "0000000",
            "D&M"       => "1000000",
            "D|A"       => "0010101",
            "D|M"       => "1010101",
            _           => unreachable!(),
        };

        let dest_bits = match dest {
            ""          => "000",
            "M"         => "001",
            "D"         => "010",
            "DM"        => "011",
            "A"         => "100",
            "AM"        => "101",
            "AD"        => "110",
            "ADM"       => "111",
            _           => unreachable!(),
        };

        let jump_bits = match jump {
            ""          => "000",
            "JGT"       => "001",
            "JEQ"       => "010",
            "JGE"       => "011",
            "JLT"       => "100",
            "JNE"       => "101",
            "JLE"       => "110",
            "JMP"       => "111",
            _           => unreachable!(),
        };

        "111".to_string() + comp_bits + dest_bits + jump_bits
    }

    fn parse_a_inst(line: &str) -> Result<Instruction, ParseError> {
        let words: Vec<&str> = line.trim().split_whitespace().collect();
        if words.len() == 1 && words[0].starts_with("@") {
            return Ok(Instruction::A(words[0].get(1..).unwrap().to_string()));
        }
        

        Err(ParseError::InvalidInstruction)
    }

    fn parse_l_inst(line: &str) -> Result<Instruction, ParseError> {
        if !(line.starts_with("(") && line.ends_with(")")) {
            return Err(ParseError::InvalidInstruction);
        }
        let identifier = line.get(1..(line.len()-1)).unwrap();
        if identifier.chars().all(|c| c.is_alphanumeric()) {
            return Ok(Instruction::L(identifier.to_string()));
        }
        Err(ParseError::InvalidInstruction)
    }

    fn parse_c_inst(line: &str) -> Result<Instruction, ParseError> {
        let line = line.trim();
        let mut current = String::from("");
        let mut c_iter = line.chars();
        let mut dest: String = String::from("");
        let mut comp: String = String::from("");
        let mut jump: String = String::from("");
        // parse dest
        while let Some(c) = c_iter.next() {
            if c == '=' {
                if current.is_empty() {
                    return Err(ParseError::InvalidInstruction);
                }
                dest = current.clone();
                current.clear();
                continue;
            }
            if c == ';' {
                comp = current.clone();
                current.clear();
                continue;
            }
            if c.is_alphanumeric() && current.chars().last().is_some_and(|last_char| last_char.is_whitespace()) {
                return Err(ParseError::InvalidInstruction);
            }
            if c.is_alphanumeric() || c == '-' || c == '+' {
                current.push(c);
                continue;
            }
            if c.is_whitespace() {
                continue;
            }
            return Err(ParseError::InvalidInstruction);
        }

        if !current.is_empty() {
            if !comp.is_empty(){
                jump = current.clone();
            }
            else {
                comp = current.clone();
            }
        }

        Ok (Instruction::C(dest, comp, jump))
    }
}
