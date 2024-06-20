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
    let mut base_names = Vec<String> = Vec::new();
    let mut grammars: Vec<Vec<String>> = Vec::new();

    // Define AST Grammars

    base_names.push("Expr");
    grammars.push(
        vec![
            "Comma    : Box<Expr> lhs, Box<Expr> rhs",
            "Ternary  : Box<Expr> cond, Box<Expr> then_expr, Box<Expr> else_expr",
            "Binary   : Box<Expr> lhs, Token op, Box<Expr> rhs",
            "Grouping : Box<Expr> expr",
            "Literal  : BOOL bool, NUMBER f64, STRING String, NIL NIL",
            "Unary    : Token op, Box<Expr> rhs",
        ].iter().map(|v| v.to_string()).collect()
    );

    base_names.push("Stmt");
    grammars.push(
        vec![
            "Expression : Box<Expr> expr",
            "Print      : Box<Expr> expr",
        ].iter().map(|v| v.to_string()).collect()
    )

    // Generate AST

    for i in 0..grammars.len() {
        let ast = match args.input_dir {
            Some(dir) => {
                GenerateAst::new(
                    dir,
                    base_names[i].to_string(),
                    grammars[i],
                )
            },
            None => {
                GenerateAst::new(
                    PathBuf::from("."),
                    base_names[i].to_string(),
                    grammars[i],
                )
            },
        };

        ast.write_ast();
    }

}
