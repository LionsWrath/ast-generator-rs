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
    let base_name = "Expr";
    let grammar = vec![
        "Comma    : Box<Expr> lhs, Box<Expr> rhs",
        "Binary   : Box<Expr> lhs, Token op, Box<Expr> rhs",
        "Grouping : Box<Expr> expr",
        "Literal  : BOOL bool, NUMBER f64, STRING String, NIL NIL",
        "Unary    : Token op, Box<Expr> rhs",
    ].iter().map(|v| v.to_string()).collect();

    let ast = match args.input_dir {
        Some(dir) => {
            GenerateAst::new(
                dir,
                base_name.to_string(),
                grammar,
            )
        },
        None => {
            GenerateAst::new(
                PathBuf::from("."),
                base_name.to_string(),
                grammar,
            )
        },
    };

    ast.write_ast();
}
