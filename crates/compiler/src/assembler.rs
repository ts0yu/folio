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
    pub body: Vec<Token<'a>>,
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
        let mut body: Vec<Token<'a>> = Vec::new();
        let mut name: &str = "";

        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Macro)?;
        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::Identifier)?;
        name = self.tokens[self.cursor.get() - 1].slice;
        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::OpenBrace)?;

        while self.tokens[self.cursor.get()].ttype != TokenType::CloseBrace {
            body.push(self.tokens[self.cursor.get()]);
            let mut curr = self.cursor.get();
            curr += 1;
            self.cursor.set(curr);
        }

        self.match_token(self.tokens[self.cursor.get()].ttype, TokenType::CloseBrace)?;

        Ok(Macro { name, body })
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

    /// Parse tokens to Opcodes.
    pub fn assemble(&self) -> Vec<Opcode> {
        let mut opcodes = Vec::new();
        for (index, token) in self.tokens.iter().enumerate() {
            match token.ttype {
                TokenType::Unknown => opcodes.push(Opcode::Unknown),
                TokenType::Allocate => opcodes.push(Opcode::Allocate),
                TokenType::Deallocate => opcodes.push(Opcode::Deallocate),
                TokenType::Claim => opcodes.push(Opcode::Claim),
                TokenType::Swap => opcodes.push(Opcode::Swap),
                TokenType::CreatePool => opcodes.push(Opcode::CreatePool),
                TokenType::CreatePair => opcodes.push(Opcode::CreatePair),
                TokenType::Jump => opcodes.push(Opcode::Jump),

                TokenType::Identifier => continue,
                TokenType::Error => continue,

                _ => panic!("test"),
            }
        }
        opcodes
    }
}
