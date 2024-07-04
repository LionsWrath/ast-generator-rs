use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

pub struct GenerateAst {
    output_dir: PathBuf,
    visitor_dir: PathBuf,
    base_name: String,
    types: Vec<String>,
}

impl GenerateAst {

    pub fn new(mut output_dir: PathBuf, base_name: String, types: Vec<String>) -> Self {

        let filename = base_name.clone() + ".rs";
        let mut visitor_dir = output_dir.clone();
        
        output_dir.push(filename);
        visitor_dir.push(base_name.clone() + "Visit.rs");

        GenerateAst {
            output_dir,
            visitor_dir,
            base_name,
            types,
        }
    }

    pub fn write_ast(&self) {

        let mut file = GenerateAst::create_file(&self.output_dir);
        let mut visitor = GenerateAst::create_file(&self.visitor_dir);
        let mut add_fmt = false;

        for t in &self.types {
            if t.contains("Literal") {
                add_fmt = true;
            }
        }

        self.add_header(&mut file, add_fmt);
        self.add_base(&mut file);
        self.add_subtypes(&mut file);
        self.add_visitor(&mut visitor);
    }

    fn create_file(filepath: &PathBuf) -> File {
        match File::create(filepath.clone()) {
            Ok(file) => file,
            Err(err) => {
                let path = filepath.display();
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

    fn add_header(&self, file: &mut File, add_fmt: bool) {
        self.write_file(file, "use crate::token::Token;\n".as_bytes());

        if add_fmt {
            self.write_file(file, "use std::fmt; \n\n".as_bytes());
        } else {
            self.write_file(file, "\n".as_bytes());
        }
    }

    fn add_base(&self, file: &mut File) {
        self.write_file(file, "#[derive(Clone, PartialEq, Debug)]\n".as_bytes());
        self.write_file(file, format!("pub enum {} {{\n", self.base_name).as_bytes());

        for t in &self.types {
            let s = t.split(':').collect::<Vec<_>>();
            let trimmed = s[0].trim();
            
            self.write_file(file, format!("    {}({}),\n", trimmed.to_uppercase(), trimmed).as_bytes());
        }

        self.write_file(file, "}\n\n".as_bytes());
    }

    fn add_subtypes(&self, file: &mut File) {
        
        for t in &self.types {
            let s = t.split(':').collect::<Vec<_>>();
            let trimmed = s[0].trim();
            let right = s[1].split(',').collect::<Vec<_>>();

            if trimmed == "Literal" {
                self.write_file(file, "#[derive(Clone, PartialEq, Debug)]\n".as_bytes());
                self.write_file(file, "pub enum Literal {\n".as_bytes());

                for r in &right {
                    let tt = r.trim().split(' ').collect::<Vec<_>>();                   

                    if tt[1] != "NIL" {
                        self.write_file(file, format!("    {}({}),\n", tt[0].to_uppercase(), tt[1]).as_bytes());
                    } else {
                        self.write_file(file, "    NIL,\n".as_bytes());
                    }
                }

                self.write_file(file, "}\n\n".as_bytes());
                self.add_display(file, "Literal".to_string(), &right);
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

    fn add_display(&self, file: &mut File, name: String, right_values: &Vec<&str>) {
        self.write_file(file, format!("impl fmt::Display for {} {{\n", name).as_bytes());
        self.write_file(file, "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n".as_bytes());
        self.write_file(file, "        match self {\n".as_bytes());

        for &r in right_values {
            let tt = r.trim().split(' ').collect::<Vec<_>>();                   

            if tt[1] != "NIL" {
                self.write_file(file, format!("            {}::{}(val) => write!(f, \"{{}}\", val),\n", name, tt[0].to_uppercase()).as_bytes());
            } else {
                self.write_file(file, format!("            {}::NIL => write!(f, \"NIL\"),\n", name).as_bytes());
            }
        }

        self.write_file(file, "        }\n    }\n}\n\n".as_bytes());
    }

    fn add_visitor(&self, file: &mut File) {
        self.write_file(file, "use crate::ast::*;\n".as_bytes());
        self.write_file(file, "use crate::token::Token;\n\n".as_bytes());

        self.write_file(file, format!("pub trait {}Visitor<T> {{\n", self.base_name).as_bytes());

        // Visitor Functions
        let lower = self.base_name.to_lowercase();
        let name = lower.chars().nth(0).unwrap();
        self.write_file(file, format!("    fn visit_{}(&mut self, {}: &{}) -> T;\n", lower, name, self.base_name).as_bytes());


        for t in &self.types {
            let s = t.split(':').collect::<Vec<_>>();
            let orig = s[0].trim();
            let lower = orig.to_lowercase();
            let name = lower.chars().nth(0).unwrap();

            self.write_file(file, format!("    fn visit_{}(&mut self, {}: &{}) -> T;\n", lower, name, orig).as_bytes());
        }

        self.write_file(file, "}\n\n".as_bytes());

    }
}
