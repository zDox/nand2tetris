use super::parser::CommandType;
use std::{ fs::write, path::Path };


pub enum CodeGenerationError {
    SaveFile,
}

pub struct CodeWriter {
    output: String,
}

impl CodeWriter {
    pub fn new() -> Self {
        Self { output: String::from("") }
    }

    pub fn write_arithmetic(&mut self, command: &str) {
    }

    // only push
    pub fn write_push_pop(&mut self, segment: &str, index: u32) {
        // First load data, which should be pushed onto the stack into the D Register
        let base_address_loc = match segment {
            "local"     => "LCL",
            "argument"  => "ARG",
            "this"      => "THIS",
            "that"      => "THAT",
            "temp" | "R13" | "R14" | "R15" => segment,
            _           => unreachable!(),
        };

        match base_address_loc {
            "temp" | "R13" | "R14" | "R15" => {
                self.output.push_str(&format!("@{}", base_address_loc));
            },
            val => {
                self.output.push_str(&format!("@{}", base_address_loc));
                self.output.push_str("A=M");
            },
        };

        self.output.push_str(&format!("A=A+{}", index));
        self.output.push_str("D=M");


        // Now set top of stack to the value of the D Register
        self.output.push_str("@SP");
        self.output.push_str("A=M");
        self.output.push_str("M=D");

        // SP++
        self.output.push_str("@SP");
        self.output.push_str("M=M+1");
    }

    pub fn save(&self, path: &Path) -> Result<(), CodeGenerationError>{
        let mut path_buf = path.to_path_buf();
        path_buf.set_extension("asm");

        match write(path_buf, self.output) {
            Ok(_) => Ok(()),
            Err(_) => Err(CodeGenerationError::SaveFile),
        }
    }
}
