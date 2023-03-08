use std::{cell::Cell, collections::HashMap};

use crate::{
    opcode::Opcode,
    token::{Token, TokenType},
};

/// Type representing an Opcode parser.
#[derive(Debug)]
pub struct Assembler<'a> {
    /// Tokens to be parsed.
    pub tokens: Vec<Token<'a>>,
    /// Cursor
    pub cursor: Cell<usize>,
}

/// Represents a macro, a reusable building block of opcodes.
#[derive(Debug, Clone)]
pub struct Macro<'a> {
    /// Macro name.
    pub name: &'a str,
    /// Body of the macro: the opcodes inside of it.
    pub body: Vec<Opcode<'a>>,
}

impl<'a> Assembler<'a> {
    /// Public constructor function that instantiates a `Parser`.
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            cursor: Cell::new(0),
        }
    }

    /// Expand all macros.
    pub fn parse_macro(&self) -> Result<Macro<'a>, ()> {
        let mut body: Vec<Opcode> = Vec::new();
        let mut name: &str = "";

        println!("{:#?}", self.tokens);

        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Macro)?;
        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Identifier)?;
        name = self.tokens[self.cursor.get() - 1].slice;
        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::OpenBrace)?;

        while self.tokens[self.cursor.get()].ttype != TokenType::CloseBrace {
            body.push(self.parse_opcode()?);
        }

        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::CloseBrace)?;

        Ok(Macro { name, body })
    }

    fn parse_opcode(&self) -> Result<Opcode<'a>, ()> {
        let current_token = self.tokens[self.cursor.get()];

        match current_token.ttype {
            TokenType::Unknown => {
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Unknown);
                Ok(Opcode::Unknown)
            }
            TokenType::Allocate => {
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Allocate);
                Ok(Opcode::Allocate)
            }
            TokenType::Deallocate => {			
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Deallocate)?;
				self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Colon)?;
				self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::UseMax)?;
				self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Colon)?;
				self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Literal)?;

				let useMax = self.tokens[self.cursor.get() - 1]
                    .slice
                    .parse::<usize>()
                    .unwrap();

				self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::PoolId)?;
				self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Colon)?;
				self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Literal)?;		

				let poolId = self.tokens[self.cursor.get() - 1]
                    .slice
                    .parse::<usize>()
                    .unwrap();

				self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::DeltaLiquidity)?;
				self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Colon)?;
				self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Literal)?;

				let deltaLiquidity = self.tokens[self.cursor.get() - 1]
                    .slice
                    .parse::<usize>()
                    .unwrap();
				
                Ok(Opcode::Deallocate { useMax, poolId, deltaLiquidity })
            }
            TokenType::Claim => {
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Claim)?;
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Colon)?;
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::PoolId)?;
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Colon)?;
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Literal)?;

                let poolId = self.tokens[self.cursor.get() - 1]
                    .slice
                    .parse::<usize>()
                    .unwrap();

                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Fee0)?;
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Colon)?;
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Literal)?;

                let fee0 = self.tokens[self.cursor.get() - 1]
                    .slice
                    .parse::<usize>()
                    .unwrap();

                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Fee1)?;
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Colon)?;
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Literal)?;

                let fee1 = self.tokens[self.cursor.get() - 1]
                    .slice
                    .parse::<usize>()
                    .unwrap();

                Ok(Opcode::Claim { poolId, fee0, fee1 })
            }
            TokenType::Swap => {
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Swap);
                Ok(Opcode::Swap)
            }
            TokenType::CreatePool => {
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::CreatePool);
                Ok(Opcode::CreatePool)
            }
            TokenType::CreatePair => {
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::CreatePair);
                Ok(Opcode::CreatePair)
            }
            TokenType::Jump => {
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Jump);
                Ok(Opcode::Jump)
            }
            TokenType::Identifier => {
                self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Identifier);
                Ok(Opcode::Identifier {
                    slice: current_token.slice,
                })
            }
            _ => panic!("something went wrong"),
        }
    }

    fn match_token(&self, actual: TokenType, expected: TokenType) -> Result<(), ()> {
        if actual == expected {
            let mut curr = self.cursor.get();
            curr += 1;
            self.cursor.set(curr);
            Ok(())
        } else {
            Err(())
        }
    }
}
