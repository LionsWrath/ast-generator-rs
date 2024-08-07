use std::path::PathBuf;

use clap::Parser;

mod ast_generator;
use ast_generator::GenerateAst;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = None)]
    input_dir: Option<std::path::PathBuf>,
}

fn main () {

    let args = Args::parse();
    let mut base_names:  Vec<String> = Vec::new();
    let mut grammars: Vec<Vec<String>> = Vec::new();

    // Define AST Grammars

    base_names.push("Expr".to_string());
    grammars.push(
        vec![
            "Comma    : Box<Expr> lhs, Box<Expr> rhs",
            "Ternary  : Box<Expr> cond, Box<Expr> then_expr, Box<Expr> else_expr",
            "Assign   : Token name, Box<Expr> value",
            "Binary   : Box<Expr> lhs, Token op, Box<Expr> rhs",
            "Grouping : Box<Expr> expr",
            "Literal  : BOOL bool, NUMBER f64, STRING String, NIL NIL",
            "Unary    : Token op, Box<Expr> rhs",
            "Variable : Token name",
        ].iter().map(|v| v.to_string()).collect()
    );

    base_names.push("Stmt".to_string());
    grammars.push(
        vec![
            "Block      : Vec<Stmt> statements",
            "Expression : Box<Expr> expr",
            "Print      : Box<Expr> expr",
            "Var        : Token name, Box<Expr> initializer",
        ].iter().map(|v| v.to_string()).collect()
    );

    // Generate AST

    for i in 0..grammars.len() {
        let ast = match args.input_dir {
            Some(ref dir) => {
                GenerateAst::new(
                    dir.clone(),
                    base_names[i].to_string(),
                    grammars[i].clone(),
                )
            },
            None => {
                GenerateAst::new(
                    PathBuf::from("."),
                    base_names[i].to_string(),
                    grammars[i].clone(),
                )
            },
        };

        ast.write_ast();
    }

}
