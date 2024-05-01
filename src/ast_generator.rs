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
        self.add_subtypes(&mut file);
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

        for t in &self.types {
            let s = t.split(':').collect::<Vec<_>>();
            let trimmed = s[0].trim();
            
            if trimmed == "Literal" {
                self.write_file(file, format!("    {}(Token),\n", trimmed.to_uppercase()).as_bytes());
            } else {
                self.write_file(file, format!("    {}({}),\n", trimmed.to_uppercase(), trimmed).as_bytes());
            }
        }

        self.write_file(file, "}\n\n".as_bytes());
    }

    fn add_subtypes(&self, file: &mut File) {
        
        for t in &self.types {
            let s = t.split(':').collect::<Vec<_>>();
            let trimmed = s[0].trim();
            let right = s[1].split(',').collect::<Vec<_>>();

            if trimmed == "Literal" {
                continue;
            }

            // Struct
            self.write_file(file, "#[derive(Clone, PartialEq, Debug)]\n".as_bytes());
            self.write_file(file, format!("pub struct {} {{\n", trimmed).as_bytes());

            for r in &right {
                let tt = r.trim().split(' ').collect::<Vec<_>>();
                self.write_file(file, format!("    pub {}: {},\n", tt[1], tt[0]).as_bytes());
            }

            self.write_file(file, "}\n\n".as_bytes());

            // Impl
            self.write_file(file, format!("impl {} {{\n", trimmed).as_bytes());
            self.write_file(file, "   pub fn new(".as_bytes());
            
            let mut params = Vec::new();
            for r in &right {
                let tt = r.trim().split(' ').collect::<Vec<_>>();
                params.push(format!("{}: {}", tt[1], tt[0]));
            }
            self.write_file(file, params.join(", ").as_bytes());
            self.write_file(file, ") -> Self {\n".as_bytes());
            self.write_file(file, format!("        {} {{\n", trimmed).as_bytes());

            for r in &right {
                let tt = r.trim().split(' ').collect::<Vec<_>>();
                self.write_file(file, format!("            {},\n", tt[1]).as_bytes());
            }
            self.write_file(file, "        }\n".as_bytes());
            self.write_file(file, "    }\n".as_bytes());
            self.write_file(file, "}\n\n".as_bytes());

        }
    }

}
