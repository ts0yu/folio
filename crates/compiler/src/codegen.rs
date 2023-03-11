struct Codegen {
    macros: HashMap<&'a str, Macro>
}

impl Codegen {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        let opcodes = Assembler::new(tokens);
        let mut macros = HashMap::new();
        
        loop {
            let curr_macro = opcodes.parse_macro().unwrap();

            match curr_macro {
                Ok(mac) => {
                    if macros.get(&mac.name).is_some() {
                        println!(
                            "{}: macro with name `{}` already exists",
                            "error".red().bold(),
                            &mac.name
                        );
                        std::process::exit(1);
                    } else {
                        macros.insert(mac.clone().name, mac);
                    }
                }
                Err(err) => break,
            }


        }

        let main_macro;

        match macros.get(&"main") {
            Some(r#main) => main_macro = r#main.clone(),
            None => panic!("no main macro found"),
        }

        Self { macros }
    }
}