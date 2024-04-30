use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

pub struct GenerateAst {
    output_dir: PathBuf,
    base_name: String,
    types: Vec<String>,
}

impl GenerateAst {

    pub fn new(mut output_dir: PathBuf, base_name: String, types: Vec<String>) -> Self {

        let filename = base_name.clone() + ".rs";
        output_dir.push(filename);

        GenerateAst {
            output_dir,
            base_name,
            types,
        }
    }

    pub fn write_ast(&self) {

        let mut file = self.create_file();

        self.add_header(&mut file);
        self.add_base(&mut file);
    }

    fn create_file(&self) -> File {
        match File::create(self.output_dir.clone()) {
            Ok(file) => file,
            Err(err) => {
                let path = self.output_dir.display();
                panic!("[AST-GEN] {err} | {path}");
            },
        }
    }
    
    fn write_file(&self, file: &mut File, text: &[u8]) {
        match file.write_all(text) {
            Ok(_) => (),
            Err(err) => {
                let path = self.output_dir.display();
                panic!("[AST-GEN] {err} | {path}");
            },
        };
    }

    fn add_header(&self, file: &mut File) {
        self.write_file(file, "use crate::token::Token;\n\n".as_bytes());
    }

    fn add_base(&self, file: &mut File) {
        self.write_file(file, "#[derive(Clone, PartialEq, Debug)]\n".as_bytes());
        self.write_file(file, format!("pub enum {} {{\n", self.base_name).as_bytes());

        for type in self.types {
            let left, right = type.split(':');
        }
    }

}
