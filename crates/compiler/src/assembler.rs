use std::{cell::Cell, collections::HashMap};

use ethers::types::{H160, Address};

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

        self.match_token(TokenType::Macro)?;
        self.match_token(TokenType::Identifier)?;
        name = self.tokens[self.cursor.get() - 1].slice;
        self.match_token(TokenType::OpenBrace)?;

        while self.tokens[self.cursor.get()].ttype != TokenType::CloseBrace {
            body.push(self.parse_opcode()?);
        }

        self.match_token(TokenType::CloseBrace)?;

        Ok(Macro { name, body })
    }

    fn parse_opcode(&self) -> Result<Opcode<'a>, ()> {
        let current_token = self.tokens[self.cursor.get()];

        match current_token.ttype {
            TokenType::Unknown => {
                self.match_token(TokenType::Unknown);
                Ok(Opcode::Unknown)
            }
            TokenType::Allocate => {
                self.match_token(TokenType::Allocate)?;
                self.match_token(TokenType::Colon)?;

                self.match_parameter(TokenType::UseMax, TokenType::Literal);
                let useMax = self.previous_literal();

                self.match_parameter(TokenType::PoolId, TokenType::Literal);
                let poolId = self.previous_literal();

                self.match_parameter(TokenType::DeltaLiquidity, TokenType::Literal);
                let deltaLiquidity = self.previous_literal();

                Ok(Opcode::Allocate {
                    useMax,
                    poolId,
                    deltaLiquidity,
                })
            }
            TokenType::Deallocate => {
                self.match_token(TokenType::Deallocate)?;
                self.match_token(TokenType::Colon)?;

                self.match_parameter(TokenType::UseMax, TokenType::Literal);
                let useMax = self.previous_literal();

                self.match_parameter(TokenType::PoolId, TokenType::Literal);
                let poolId = self.previous_literal();

                self.match_parameter(TokenType::DeltaLiquidity, TokenType::Literal);
                let deltaLiquidity = self.previous_literal();

                Ok(Opcode::Deallocate {
                    useMax,
                    poolId,
                    deltaLiquidity,
                })
            }
            TokenType::Claim => {
                self.match_token(TokenType::Claim)?;
                self.match_token(TokenType::Colon)?;

                self.match_parameter(TokenType::PoolId, TokenType::Literal);
                let poolId = self.previous_literal();

                self.match_parameter(TokenType::Fee0, TokenType::Literal);
                let fee0 = self.previous_literal();

                self.match_parameter(TokenType::Fee1, TokenType::Literal);
                let fee1 = self.previous_literal();

                Ok(Opcode::Claim { poolId, fee0, fee1 })
            }
            TokenType::Swap => {
                self.match_token(TokenType::Swap)?;
                self.match_token(TokenType::Colon)?;

                self.match_parameter(TokenType::UseMax, TokenType::Literal);
                let useMax = self.previous_literal();

                self.match_parameter(TokenType::PoolId, TokenType::Literal);
                let poolId = self.previous_literal();

                self.match_parameter(TokenType::Amount0, TokenType::Literal);
                let amount0 = self.previous_literal();

                self.match_parameter(TokenType::Amount1, TokenType::Literal);
                let amount1 = self.previous_literal();

                self.match_parameter(TokenType::SellAsset, TokenType::Literal);
                let sellAsset = self.previous_literal();

                Ok(Opcode::Swap {
                    useMax,
                    poolId,
                    amount0,
                    amount1,
                    sellAsset,
                })
            }
            TokenType::CreatePool => {
                self.match_token(TokenType::CreatePool);
                self.match_token(TokenType::Colon)?;

                self.match_parameter(TokenType::PairId, TokenType::Literal);
                let pair_id = self.previous_literal();

                self.match_parameter(TokenType::Controller, TokenType::AddressLiteral);
    
                let controller = self.tokens[self.cursor.get() - 1]
                    .slice
                    .parse::<Address>()
                    .unwrap();

                self.match_parameter(TokenType::PriorityFee, TokenType::Literal);        
                let priority_fee = self.previous_literal();

                self.match_parameter(TokenType::Fee, TokenType::Literal);            
                let fee = self.previous_literal();

                self.match_parameter(TokenType::Vol, TokenType::Literal);               
                let vol = self.previous_literal();

                self.match_parameter(TokenType::Dur, TokenType::Literal);                   
                let dur = self.previous_literal();

                self.match_parameter(TokenType::Jit, TokenType::Literal);
                let jit = self.previous_literal();

                self.match_parameter(TokenType::MaxPrice, TokenType::Literal);                            
                let max_price = self.previous_literal();

                self.match_parameter(TokenType::Price, TokenType::Literal);                               
                let price = self.previous_literal();

                Ok(Opcode::CreatePool)
            }
            TokenType::CreatePair => {
                self.match_token(TokenType::CreatePair);
                self.match_token(TokenType::Colon)?;

                self.match_parameter(TokenType::Token0, TokenType::AddressLiteral);

                let token0 = self.tokens[self.cursor.get() - 1]
                    .slice
                    .parse::<Address>()
                    .unwrap();

                self.match_parameter(TokenType::Token1, TokenType::AddressLiteral);
    
                let token1 = self.tokens[self.cursor.get() - 1]
                    .slice
                    .parse::<Address>()
                    .unwrap();

                Ok(Opcode::CreatePair { token0, token1 })
            }
            TokenType::Jump => {
                self.match_token(TokenType::Jump);
                Ok(Opcode::Jump)
            }
            TokenType::Identifier => {
                self.match_token(TokenType::Identifier);
                Ok(Opcode::Identifier {
                    slice: current_token.slice,
                })
            }
            _ => panic!("something went wrong"),
        }
    }

    fn match_token(&self, expected: TokenType) -> Result<(), ()> {
        if self.tokens[self.cursor.get()].ttype == expected {
            let mut curr = self.cursor.get();
            curr += 1;
            self.cursor.set(curr);
            Ok(())
        } else {
            Err(())
        }
    }

    fn parse_parameter(&self, key: TokenType, value: TokenType) -> Result<(), ()> {
        self.match_token(key)?;
        self.match_token(TokenType::Colon)?;
        self.match_token(value)?;

        Ok(())
    }

    fn previous_literal(&self, key: TokenType, value: TokenType) -> Result<usize, ()> {
        let literal = self.tokens[self.cursor.get() - 1]
            .slice
            .parse::<usize>()
            .unwrap();

        Ok(literal)
    }
}
