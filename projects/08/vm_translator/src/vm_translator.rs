use std::path::Path;

#[path = "parser.rs"]
mod parser;
use parser::{ Parser, CommandType };
#[path = "code_writer.rs"]
mod code_writer;
use code_writer::CodeWriter;

pub struct VMTranslator {
    path: Box<Path>,
    code_writer: CodeWriter,
}

impl VMTranslator {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.into(),
            code_writer: CodeWriter::new(path.file_stem().unwrap().to_str().unwrap()),
        }
    }

    pub fn translate(&mut self) {
        if self.path.is_dir() { 
            self.translate_folder(); 
            let mut new_out_path = self.path.to_path_buf();
            new_out_path.push(self.path.file_stem().unwrap().to_str().unwrap());
            self.path = new_out_path.into();
        }
        else if self.path.is_file() { self.translate_file(&self.path.clone()); }


        self.code_writer.write_exit_code();
        let _res = self.code_writer.save(&self.path);
    }

    fn translate_folder(&mut self) {
        println!("Translating folder '{}'", self.path.display());
        let paths = self.path.read_dir().expect("Could not read folder");

        self.code_writer.write_init_code();

        for path in paths {
            let file_path = path.unwrap().path();
            if file_path.extension().unwrap() == "vm" {
                self.code_writer.set_file_name(file_path.file_stem().unwrap().to_str().unwrap());
                self.translate_file(&file_path);
            }
        }
    }


    fn translate_file(&mut self, file_path: &Path) {
        let mut parser = Parser::new(file_path).expect("File has wrong Filetype: expeceted .vm");
        while parser.has_more_lines() {
            parser.advance();
            let _res = match parser.command_type().expect("Failed determining CommandType"){
                CommandType::Pop => self.code_writer.write_pop(
                    parser.arg1().expect("No segement specified when using pop command"), 
                    parser.arg2().expect("No index specified when using pop command")),
                CommandType::Push => self.code_writer.write_push(
                    parser.arg1().expect("No segment specified when usign push command"), 
                    parser.arg2().expect("No index specified when using push command")),
                CommandType::Arithmetic => self.code_writer.write_arithmetic(parser.arg1().expect("No Arithmetic command")),
                CommandType::Label => self.code_writer.write_label(parser.arg1().expect("No Label specified when using label command")),
                CommandType::Goto => self.code_writer.write_goto(parser.arg1().expect("No Label specified when using goto command")),
                CommandType::IfGoto => self.code_writer.write_if_goto(parser.arg1().expect("No Label specified when using if-goto command")),
                CommandType::Function => self.code_writer.write_function(
                    parser.arg1().expect("No function name specified when using function command"),
                    parser.arg2().expect("No amount of variables specified when using function command")),
                CommandType::Call => self.code_writer.write_call(
                    parser.arg1().expect("No function name specified when using call command"),
                    parser.arg2().expect("No amount of arguments specified when using call command")),
                CommandType::Return => self.code_writer.write_return(),
                CommandType::None => unreachable!(),
            };
        }

        println!("Translation of file '{}' completed", file_path.file_name().unwrap().to_str().unwrap());
    }
}
