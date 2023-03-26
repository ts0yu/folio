use std::fs;

use clap::{Parser, Subcommand};
use compiler::{assembler::Assembler, codegen::Codegen, token::Token};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build { path: String },
}

fn main() {
    let value = Value::parse();

    match &value.command {
        Commands::Build { path } => {
            let start = std::time::Instant::now();

            let contents = fs::read_to_string(path).unwrap();
            let tokens = Token::lex(&contents);
            let expressions = Assembler::parse(tokens);

            let codegen = Codegen::new(expressions);
            let encoded = codegen.encode();

            let duration = start.elapsed();

            println!("Compilation finished in: {duration:?}");

            println!("{encoded:#?}");
        }
    }
}
