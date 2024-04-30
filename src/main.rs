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
        "Binary   : Expr lhs, Token op, Expr rhs",
        "Grouping : Expr expr",
        "Literal  : Token token",
        "Unary    : token op, Expr rhs",
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
