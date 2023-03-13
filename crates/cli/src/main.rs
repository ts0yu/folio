use std::{collections::HashMap, fs};

use clap::{Parser, Subcommand};
use colored::*;
use compiler::{
    assembler::{Assembler, Expression, Macro},
    codegen::Codegen,
    token::{Token, TokenType},
};

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
            let contents = fs::read_to_string(path).unwrap();
            let tokens = Token::lex(&contents);
            let mut _main_macro: Macro = Macro {
                name: "",
                body: Vec::new(),
            };

            let mut occurences: usize = 0;

            tokens.clone().into_iter().for_each(|token| {
                if token.ttype == TokenType::Macro {
                    occurences += 1
                }
            });

            let opcodes = Assembler::new(tokens);
            let mut macros = HashMap::new();

            for _ in 0..occurences {
                let mac = opcodes.parse_macro().unwrap();

                if macros.get(&mac.name).is_some() {
                    println!(
                        "{}: macro with name `{}` already exists",
                        "error".red().bold(),
                        &mac.name
                    );
                    std::process::exit(1);
                }

                if macros.get(&mac.name).is_none() {
                    macros.insert(mac.clone().name, mac);
                }
            }

            match macros.get(&"main") {
                Some(r#main) => _main_macro = r#main.clone(),
                None => panic!("no main macro found"),
            }

            loop {
                let mut invocation_found = false;

                for (i, n) in _main_macro.clone().body.iter().enumerate() {
                    match n {
                        Expression::Invocation(slice) => {
                            let replacer = macros.get(slice);
                            let mut index: usize = i;

                            _main_macro.body.remove(i);

                            for g in &replacer.unwrap().body {
                                _main_macro.body.insert(index, g.clone());
                                index += 1;
                            }

                            invocation_found = true;
                        }
                        _ => continue,
                    }
                }

                if !invocation_found {
                    break;
                }
            }

            println!("{} `{}`", "Compiling".green().bold(), path);

            let exprs = _main_macro.body;

            let codegen = Codegen::new(exprs);
            let encoded = codegen.encode();

            println!("{encoded:#?}");

            // println!("Opcodes: {opcodes:#?}");
        }
    }
}
