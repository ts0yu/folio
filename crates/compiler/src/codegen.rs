use std::collections::HashMap;

use bytes::Bytes;

use crate::{
    assembler::{Assembler, Expression, Macro},
    token::Token,
};

pub struct Codegen<'a> {
    macros: HashMap<&'a str, Macro<'a>>,
}

impl<'a> Codegen<'a> {
    /// Public constructor function to instantiate a `Codegen`.
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        let opcodes = Assembler::new(tokens);
        let mut macros = HashMap::new();

        loop {
            let curr_macro = opcodes.parse_macro();

            match curr_macro {
                Ok(mac) => {
                    if macros.get(&mac.name).is_some() {
                        std::process::exit(1);
                    } else {
                        macros.insert(mac.clone().name, mac);
                    }
                }
                Err(_) => break,
            }
        }

        match macros.get(&"main") {
            Some(_main) => (),
            None => panic!("no main macro found"),
        }

        Self { macros }
    }

    /// Expand all macros and encode into hex, ready to be executed on the FVM.
    pub fn encode(&self) -> Bytes {
        let mut main_macro;

        match self.macros.get(&"main") {
            Some(r#main) => main_macro = r#main.clone(),
            None => panic!("no main macro found"),
        }

        for (i, n) in main_macro.clone().body.iter().enumerate() {
            match n {
                Expression::Invocation(slice) => {
                    let replacer = self.macros.get(slice);
                    let mut index: usize = i;

                    for g in &replacer.unwrap().body {
                        main_macro.body.insert(index, g.clone());
                        index += 1;
                    }
                }
                _ => continue,
            }
        }

        // do the actual encoding stuff here;

        Bytes::new()
    }
}
