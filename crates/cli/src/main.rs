use std::{collections::HashMap, fs, time::Instant};

use clap::{Parser, Subcommand};
use colored::*;
use compiler::{
    assembler::{Assembler, Expression, Macro},
    opcode::Opcode,
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
            let mut main_macro: Macro = Macro {
                name: "",
                body: Vec::new(),
            };

            let mut occurences: usize = 0;

            tokens.clone().into_iter().for_each(|token| {
                if token.ttype == TokenType::Macro {
                    occurences += 1
                }
            });



            for _ in 0..occurences {
                let mac = opcodes.parse_macro().unwrap();
                println!("{:#?}", mac);

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
                Some(r#main) => main_macro = r#main.clone(),
                None => panic!("no main macro found"),
            }

            for (i, n) in main_macro.clone().body.iter().enumerate() {
                match n {
                    Expression::Invocation(slice) => {
                        let replacer = macros.get(slice);
                        let mut index: usize = i;

                        for g in &replacer.unwrap().body {
                            main_macro.body.insert(index, g.clone());
                            index += 1;
                        }
                    }
                    _ => continue,
                }
            }

            println!("{} `{}`", "Compiling".green().bold(), path);

            let now = Instant::now();
            let opcodes = main_macro.body;

            println!("Opcodes: {opcodes:#?}");
        }
    }
}
