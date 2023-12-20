use std::{ fs::write, path::Path };


pub enum CodeGenerationError {
    SaveFile,
}

pub struct CodeWriter {
    output: String,
    line_number: u32,
}

impl CodeWriter {
    pub fn new() -> Self {
        Self { output: String::from(""), line_number: 0 }
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        self.write_comment(&format!("Start: Arithmetic command '{}'", command));
        // Pop either one or two elements from the stack depending on which command
         match command {
            "add" | "sub" | "eq" | "gt" | "lt" | "and" | "or"   => {
                self.write_pop("R13", 0); // y in R13
                self.write_pop("R14", 0); // x in R14
                true
            },
            "neg" | "not"                                       => {
                self.write_pop("R13", 0); // y in R13
                false
            },
            unexpected                                          => panic!("unexpected arithmetic command '{}'", unexpected),
        };

        self.write_code("@R13");
        self.write_code("D=M");

        self.write_code("@R14");

        // M = x, D = y

        match command {
            "add"   => self.write_code("M=M+D"),
            "sub"   => self.write_code("M=M-D"),
            "neg"   => self.write_code("M=-D"),
            "eq"    => self.write_comparison("JEQ"),
            "gt"    => self.write_comparison("JGT"),
            "lt"    => self.write_comparison("JLT"),
            "and"   => self.write_code("M=M&D"),
            "or"    => self.write_code("M=M|D"),
            "not"   => self.write_code("M=!D"),
            _ => unreachable!(),
        }

        // Result of operation is now at R14
        // Need to push value of R14 onto the stack

        self.write_push("R14", 0);

        self.write_comment(&format!("End: Arithmetic command '{}'", command));
        self.write_empty_line();
    }


    // only push
    pub fn write_push(&mut self, segment: &str, index: i32) {
        self.write_comment(&format!("Start: Push command 'segment: {} index: {}'", segment, index));
        // First load data, which should be pushed onto the stack into the D Register

        let base_address_loc = Self::get_ram_location_for_segment(segment);
        match base_address_loc {
            "TEMP" => self.write_code("@5"),
            "R13" | "R14" | "R15" => {
                self.write_code(&format!("@{}", base_address_loc));
            },
            "constant" => {
                self.write_code(&format!("@{}", index));
                self.write_code("D=A");
            },
            _ => {
                self.write_code(&format!("@{}", base_address_loc));
                self.write_code("A=M");
            },
        };

        // if segment is not argument than is in the a registry the base address
        if base_address_loc != "constant"{
            if index == 1 {
                self.write_code("A=A+1");
            }
            else if index == -1 {
                self.write_code("A=A-1");
            }
            else if index != 0 {
                self.write_code("D=A");
                self.write_code(&format!("@{}", index));
                self.write_code("A=A+D");
            }

            self.write_code("D=M");
        }


        // Now set top of stack to the value of the D Register
        self.write_code("@SP");
        self.write_code("A=M");
        self.write_code("M=D");
        
        self.increment_sp_by(1);

        self.write_comment(&format!("End: Push command 'segment: {} index: {}'", segment, index));
        self.write_empty_line();
    }


    pub fn write_pop(&mut self, segment: &str, index: i32) {
        self.write_comment(&format!("Start: Pop command 'segment: {} index: {}'", segment, index));
        // First load data, which should be pop from the stack into the D Register
        //

        self.increment_sp_by(-1);

        let base_address_loc = Self::get_ram_location_for_segment(segment);




        match base_address_loc {

            "TEMP" => self.write_code("@5"),
            "R13" | "R14" | "R15" => {
                self.write_code(&format!("@{}", base_address_loc));
            },
            "constant" => {
                self.write_code(&format!("@{}", index));
            }
            _ => {
                self.write_code(&format!("@{}", base_address_loc));
                self.write_code("A=M");
            },
        };

        // if segment is not argument than is in the a registry the base address
        if base_address_loc != "constant" {
            if index == 1 {
                self.write_code("A=A+1");
            }
            else if index == -1 {
                self.write_code("A=A-1");
            }
            else if index != 0 {
                self.write_code("D=A");
                self.write_code(&format!("@{}", index));
                self.write_code("A=A+D");
            }
        }

        self.write_code("D=A");
        self.write_code("@R15");
        self.write_code("M=D");


        self.write_code("@SP");
        self.write_code("A=M");
        self.write_code("D=M");

        self.write_code("@R15");
        self.write_code("A=M");
        
        self.write_code("M=D");

        self.write_comment(&format!("End: Pop command 'segment: {} index: {}'", segment, index));
        self.write_empty_line();
    }

    fn get_ram_location_for_segment(segment: &str) -> &str {
        match segment {
            "local"     => "LCL",
            "argument"  => "ARG",
            "this"      => "THIS",
            "that"      => "THAT",
            "temp"      => "TEMP",
            "constant" | "R13" | "R14" | "R15" => segment,
            _           => unreachable!(),
        }
    }

    fn write_comparison(&mut self, jump_cond: &str) {
        self.write_code("D=M-D");
        self.write_code(&format!("@{}", self.line_number+6));
        self.write_code(&format!("D;{}", jump_cond));
        self.write_code("@R14");
        self.write_code("M=0");
        self.write_code(&format!("@{}", self.line_number+4));
        self.write_code("0; JMP");
        self.write_code("@R14");
        self.write_code("M=-1");
    }

    fn increment_sp_by(&mut self, val: i32) {
        let val_out = if val >= 0 { format!("+{}", val) } else { val.to_string() };
        self.write_comment(&format!("// SP = SP {}", val_out));
        self.write_code("@SP");
        self.write_code(&format!("M=M{}", val_out));
    }

    fn write_empty_line(&mut self) {
        self.write("\n");
    }

    fn write_comment(&mut self, comment: &str) {
        self.write(&format!("// {} \n", comment));
    }

    fn write_code(&mut self, asm_command: &str) {
        self.line_number += 1;
        self.write(asm_command);
    }

    fn write(&mut self, asm_command: &str) {
        self.output.push_str(&format!("{}\n", asm_command));
    }

    pub fn save(&self, path: &Path) -> Result<(), CodeGenerationError>{
        let mut path_buf = path.to_path_buf();
        path_buf.set_extension("asm");

        match write(path_buf, &self.output) {
            Ok(_) => Ok(()),
            Err(_) => Err(CodeGenerationError::SaveFile),
        }
    }
}
