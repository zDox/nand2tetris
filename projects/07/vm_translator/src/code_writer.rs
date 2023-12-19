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
        self.write_comment(&format!("Arithmetic command '{}'", command));
        // Pop either one or two elements from the stack depending on which command
        let two_numbers_operation = match command {
            "add" | "sub" | "eq" | "gt" | "lt" | "and" | "or"   => {
                self.write_pop("R13", 0);
                self.write_pop("R14", 0);
                true
            },
            "neg" | "not"                                       => {
                self.write_pop("R13", 0);
                false
            },
            unexpected                                          => panic!("unexpected arithmetic command '{}'", unexpected),
        };

        if two_numbers_operation {
            self.write("@R13");
            self.write("D=M");
        }

        self.write("@R14");

        match command {
            "add"   => self.write("M=M+D"),
            "sub"   => self.write("M=M+D"),
            "neg"   => self.write("M=-D"),
            "eq"    => unimplemented!(),
            "gt"    => unimplemented!(),
            "lt"    => unimplemented!(),
            "and"   => self.write("M=M&D"),
            "or"    => self.write("M=M|D"),
            "not"   => self.write("M=!M"),
            _ => unreachable!(),
        }

        // Result of operation is now at R14
        // Need to push value of R14 onto the stack

        self.write_push("R14", 0);

        self.write_empty_line();
    }


    // only push
    pub fn write_push(&mut self, segment: &str, index: i32) {
        self.write_comment(&format!("Push command 'segment: {} index: {}'", segment, index));
        // First load data, which should be pushed onto the stack into the D Register

        let base_address_loc = Self::get_ram_location_for_segment(segment);
        match base_address_loc {
            "temp" | "R13" | "R14" | "R15" => {
                self.write(&format!("@{}", base_address_loc));
            },
            "constant" => {
                self.write(&format!("@{}", index));
                self.write("D=A");
            },
            _ => {
                self.write(&format!("@{}", base_address_loc));
                self.write("A=M");
            },
        };

        // if segment is not argument than is in the a registry the base address
        if base_address_loc != "constant" {
            self.write(&format!("A=A+{}", index));
            self.write("D=M");
        }


        // Now set top of stack to the value of the D Register
        self.write("@SP");
        self.write("A=M");
        self.write("M=D");
        
        self.increment_sp_by(1);

        self.write_empty_line();
    }


    pub fn write_pop(&mut self, segment: &str, index: i32) {
        self.write_comment(&format!("Pop command 'segment: {} index: {}'", segment, index));
        // First load data, which should be pop from the stack into the D Register

        let base_address_loc = Self::get_ram_location_for_segment(segment);


        self.write("@SP");
        self.write("A=M");
        self.write("D=M");

        match base_address_loc {
            "temp" | "R13" | "R14" | "R15" => {
                self.write(&format!("@{}", base_address_loc));
            },
            _ => {
                self.write(&format!("@{}", base_address_loc));
                self.write("A=M");
            },
        };

        // if segment is not argument than is in the a registry the base address
        if base_address_loc != "constant" {
            self.write(&format!("A=A+{}", index));
        }
        
        self.write("M=D");
        self.increment_sp_by(-1);

        self.write_empty_line();
    }

    fn get_ram_location_for_segment(segment: &str) -> &str {
        match segment {
            "local"     => "LCL",
            "argument"  => "ARG",
            "this"      => "THIS",
            "that"      => "THAT",
            "temp" | "constant" | "R13" | "R14" | "R15" => segment,
            _           => unreachable!(),
        }
    }

    fn increment_sp_by(&mut self, val: i32) {
        let val_out = if val >= 0 { format!("+{}", val) } else { format!("-{}", val) };
        self.write(&format!("// SP = SP {}", val_out));
        self.write("@SP");
        self.write(&format!("// M=M{}", val_out));
    }

    fn write_empty_line(&mut self) {
        self.write("\n");
    }

    fn write_comment(&mut self, comment: &str) {
        self.write(&format!("// {}", comment));
    }

    fn write(&mut self, asm_command: &str) {
        self.output.push_str(asm_command);
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
