use std::{ fs::write, path::Path };

mod symbol_table;
use symbol_table::SymbolTable;

pub enum CodeGenerationError {
    SaveFile,
}

pub struct CodeWriter {
    file_name: String,
    output: String,
    line_number: u32,
    static_table: SymbolTable,
}

impl CodeWriter {
    pub fn new(file_name: &str) -> Self {
        Self { 
            file_name: file_name.to_string(),
            output: String::from(""), 
            line_number: 0, 
            static_table: SymbolTable::new(15)
        }
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        self.write_comment(&format!("Start: Arithmetic command '{}'", command));

        // Pop always one if Arithmetic operation operates on only one
        // Pop a second one (x) if Arithmetic operation operates on two
        self.write_pop("R13", 0); // y in R13
        if ["add", "sub", "eq", "gt", "lt", "and", "or"].contains(&command) {
                self.write_pop("R14", 0); // x in R14
        }

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
            "static" => {
                let addr = self.static_table.get_symbol(&format!("{}.{}", self.file_name, index.to_string()));
                self.write_code(&format!("@{}", addr));
                self.write_code("D=M");
            },
            "TEMP" => self.write_code("@5"),
            "pointer" => self.write_code("@3"),
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
        if !["constant", "static"].contains(&base_address_loc){
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
            "static" => {
                let addr = self.static_table.get_symbol(&format!("{}.{}", self.file_name, index.to_string()));
                self.write_code(&format!("@{}", addr));
            },
            "pointer" => self.write_code("@3"),
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
        if !["constant", "static"].contains(&base_address_loc) {
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

    pub fn write_label(&mut self, label: &str) {
        self.write_code(&format!("({}.{})", self.file_name, label));
    }

    pub fn write_goto(&mut self, label: &str) {
        self.write_code(&format!("@{}.{}", self.file_name, label));
        self.write_code("0;JMP");
    }

    pub fn write_if_goto(&mut self, label: &str) {
        self.write_comment(&format!("Start: IF_GOTO label: {}", label));
        self.write_pop("R13", 0);
        self.write_code("D=M");
        self.write_code(&format!("@{}.{}", self.file_name, label));
        self.write_code("D;JGT");
        self.write_comment(&format!("End: IF_GOTO label: {}", label));
    }

    pub fn write_function(&mut self, fn_name: &str, n_vars: i32) {
        todo!();
    }

    pub fn write_call(&mut self, fn_name: &str, n_args: i32) {
        todo!();
    }

    pub fn write_return(&mut self) {
        todo!();
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

    pub fn write_bootstrap_code(&mut self) {
        self.write_comment("Start: Bootstrap Code");

        // Load 256 into SP
        self.write_code("@256");
        self.write_code("D=A");
        self.write_code("@SP");
        self.write_code("M=D");

        // call Sys.init
        self.write_call("Sys.init", 0);
        self.write_comment("End: Bootstrap Code");
    }

    fn increment_sp_by(&mut self, val: i32) {
        let val_out = if val >= 0 { format!("+{}", val) } else { val.to_string() };
        self.write_comment(&format!("SP = SP {}", val_out));
        self.write_code("@SP");
        self.write_code(&format!("M=M{}", val_out));
        self.write_comment(&format!("Start of increment: SP = SP {}", val_out));
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

    pub fn set_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
        self.write_comment(&format!("File: '{}'", file_name));
        self.write_empty_line();
    }

    pub fn save(&self, path: &Path) -> Result<(), CodeGenerationError>{
        let mut path_buf = path.to_path_buf();
        path_buf.set_extension("asm");

        match write(path_buf, &self.output) {
            Ok(_) => Ok(()),
            Err(_) => Err(CodeGenerationError::SaveFile),
        }
    }

    fn get_ram_location_for_segment(segment: &str) -> &str {
        match segment {
            "local"     => "LCL",
            "argument"  => "ARG",
            "this"      => "THIS",
            "that"      => "THAT",
            "temp"      => "TEMP",
            "pointer"   => "pointer",
            "static"    => "static",
            "constant" | "R13" | "R14" | "R15" => segment,
            _           => unreachable!(),
        }
    }
}
