use std::{
    fs::write,
    path::{Path, PathBuf},
};

pub enum Segment {
    ARGUMENT,
    CONSTANT,
    LOCAL,
    STATIC,
    THIS,
    THAT,
    POINTER,
    TEMP,
}

impl Segment {
    fn as_str(&self) -> &str {
        match self {
            Self::ARGUMENT => "arg",
            Self::CONSTANT => "constant",
            Self::LOCAL => "local",
            Self::STATIC => "static",
            Self::THIS => "this",
            Self::THAT => "that",
            Self::POINTER => "pointer",
            Self::TEMP => "temp",
        }
    }
}


pub enum ArithmeticCommand {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT
}

impl ArithmeticCommand {
    fn as_str(&self) -> &str {
        match self {
            Self::ADD => "add",
            Self::SUB => "sub",
            Self::NEG => "neg",
            Self::EQ  => "eq",
            Self::GT  => "gt",
            Self::LT  => "lt",
            Self::AND  => "and",
            Self::OR  => "or",
            Self::NOT  => "or",
        }
    }
}


pub struct VMWriter {
    file_path: PathBuf,
    output: String,
}

impl VMWriter {
    pub fn new(output_path: &Path) -> Self{
        Self {
            file_path: output_path.to_path_buf(),
            output: "".to_string(),
        }
    }

    pub fn close(&self) {
        write(&self.file_path, &self.output).expect("Could not write file");
    }

    pub fn write(&mut self, line: &str) {
        self.output.push_str(&(line.to_string() + "\n"));
    }

    pub fn write_push(&mut self, segment: &Segment, index: u32) {
        self.write(&format!("push {} {}", segment.as_str(), index));
    }

    pub fn write_pop(&mut self, segment: &Segment, index: u32) {
        self.write(&format!("pop {} {}", segment.as_str(), index));
    }

    pub fn write_arithmetic(&mut self, command: &ArithmeticCommand) {
        self.write(command.as_str());
    }

    pub fn write_label(&mut self, label: &str) {
        self.write(&format!("label {}", label));
    }
    
    pub fn write_goto(&mut self, label: &str) {
        self.write(&format!("goto {}", label));
    }

    pub fn write_if(&mut self, label: &str) {
        self.write(&format!("if {}", label));
    }

    pub fn write_call(&mut self, name: &str, n_args: u32) {
        self.write(&format!("call {} {}", name, n_args));
    }

    pub fn write_function(&mut self, name: &str, n_args: u32) {
        self.write(&format!("function {} {}", name, n_args));
    }

    pub fn write_return(&mut self) {
        self.write("return");
    }
}
