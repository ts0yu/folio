use std::cell::Cell;

use ethers::types::Address;

use crate::{
    opcode::Opcode,
    token::{Token, TokenType},
};

/// # Assembler
///
/// The parsing module of Folio.
/// Convert a [`Vec<Token>`] into an more abstract representation which can be used to generate FVM bytecode.

/// Type representing an Opcode parser.
#[derive(Debug)]
pub struct Assembler<'a> {
    /// Tokens to be parsed.
    pub tokens: Vec<Token<'a>>,
    /// Cursor
    pub cursor: Cell<usize>,
}

/// Represents an expression.
/// An expression is a node in the AST, parsed inside macros.
#[derive(Clone, Debug)]
pub enum Expression<'a> {
    /// An opcode.
    Opcode(Opcode),
    /// A macro invocation.
    Invocation(&'a str),
}

/// Represents a macro, a reusable building block of opcodes.
#[derive(Debug, Clone)]
pub struct Macro<'a> {
    /// Macro name.
    pub name: &'a str,
    /// Body of the macro: the opcodes inside of it.
    pub body: Vec<Expression<'a>>,
}

impl<'a> Assembler<'a> {
    /// Public constructor function that instantiates a `Parser`.
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            cursor: Cell::new(0),
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

    fn previous_literal(&self) -> Result<usize, ()> {
        let literal = self.tokens[self.cursor.get() - 1]
            .slice
            .parse::<usize>()
            .unwrap();

        Ok(literal)
    }

    /// Expand all macros.
    pub fn parse_macro(&self) -> Result<Macro<'a>, ()> {
        let mut body: Vec<Expression> = Vec::new();
        let name: &str;

        self.match_token(TokenType::Macro)?;
        self.match_token(TokenType::Identifier)?;
        name = self.tokens[self.cursor.get() - 1].slice;
        self.match_token(TokenType::OpenBrace)?;

        while self.tokens[self.cursor.get()].ttype != TokenType::CloseBrace {
            body.push(self.parse_expression()?);
        }

        self.match_token(TokenType::CloseBrace)?;

        Ok(Macro { name, body })
    }

    fn parse_expression(&self) -> Result<Expression<'a>, ()> {
        let current_token = self.tokens[self.cursor.get()];

        match current_token.ttype {
            TokenType::Unknown => {
                self.match_token(TokenType::Unknown)?;
                Ok(Expression::Opcode(Opcode::Unknown))
            }
            TokenType::Allocate => Ok(Expression::Opcode(self.allocate()?)),
            TokenType::Deallocate => Ok(Expression::Opcode(self.deallocate()?)),
            TokenType::Claim => Ok(Expression::Opcode(self.claim()?)),
            TokenType::Swap => Ok(Expression::Opcode(self.swap()?)),
            TokenType::CreatePool => Ok(Expression::Opcode(self.create_pool()?)),
            TokenType::CreatePair => Ok(Expression::Opcode(self.create_pair()?)),
            TokenType::Jump => {
                self.match_token(TokenType::Jump)?;
                Ok(Expression::Opcode(Opcode::Jump))
            }
            TokenType::Identifier => {
                self.match_token(TokenType::Identifier)?;
                Ok(Expression::Invocation(current_token.slice))
            }
            _ => panic!("something went wrong"),
        }
    }

    fn create_pair(&self) -> Result<Opcode, ()> {
        self.match_token(TokenType::CreatePair)?;
        self.match_token(TokenType::Colon)?;

        self.parse_parameter(TokenType::Token0, TokenType::AddressLiteral)?;

        let token_0 = self.tokens[self.cursor.get() - 1]
            .slice
            .parse::<Address>()
            .unwrap();

        self.parse_parameter(TokenType::Token1, TokenType::AddressLiteral)?;

        let token_1 = self.tokens[self.cursor.get() - 1]
            .slice
            .parse::<Address>()
            .unwrap();

        Ok(Opcode::CreatePair { token_0, token_1 })
    }

    fn create_pool(&self) -> Result<Opcode, ()> {
        self.match_token(TokenType::CreatePool)?;
        self.match_token(TokenType::Colon)?;

        self.parse_parameter(TokenType::PairId, TokenType::Literal)?;
        let pair_id = self.previous_literal()?;

        self.parse_parameter(TokenType::Controller, TokenType::AddressLiteral)?;

        let controller = self.tokens[self.cursor.get() - 1]
            .slice
            .parse::<Address>()
            .unwrap();

        self.parse_parameter(TokenType::PriorityFee, TokenType::Literal)?;
        let priority_fee = self.previous_literal()?;

        self.parse_parameter(TokenType::Fee, TokenType::Literal)?;
        let fee = self.previous_literal()?;

        self.parse_parameter(TokenType::Vol, TokenType::Literal)?;
        let vol = self.previous_literal()?;

        self.parse_parameter(TokenType::Dur, TokenType::Literal)?;
        let dur = self.previous_literal()?;

        self.parse_parameter(TokenType::Jit, TokenType::Literal)?;
        let jit = self.previous_literal()?;

        self.parse_parameter(TokenType::MaxPrice, TokenType::Literal)?;
        let max_price = self.previous_literal()?;

        self.parse_parameter(TokenType::Price, TokenType::Literal)?;
        let price = self.previous_literal()?;

        Ok(Opcode::CreatePool {
            pair_id,
            controller,
            priority_fee,
            fee,
            vol,
            dur,
            jit,
            max_price,
            price,
        })
    }

    fn swap(&self) -> Result<Opcode, ()> {
        self.match_token(TokenType::Swap)?;
        self.match_token(TokenType::Colon)?;

        self.parse_parameter(TokenType::UseMax, TokenType::Literal)?;
        let use_max = self.previous_literal()?;

        self.parse_parameter(TokenType::PoolId, TokenType::Literal)?;
        let pool_id = self.previous_literal()?;

        self.parse_parameter(TokenType::Amount0, TokenType::Literal)?;
        let amount_0 = self.previous_literal()?;

        self.parse_parameter(TokenType::Amount1, TokenType::Literal)?;
        let amount_1 = self.previous_literal()?;

        self.parse_parameter(TokenType::SellAsset, TokenType::Literal)?;
        let sell_asset = self.previous_literal()?;

        Ok(Opcode::Swap {
            use_max,
            pool_id,
            amount_0,
            amount_1,
            sell_asset,
        })
    }

    fn claim(&self) -> Result<Opcode, ()> {
        self.match_token(TokenType::Claim)?;
        self.match_token(TokenType::Colon)?;

        self.parse_parameter(TokenType::PoolId, TokenType::Literal)?;
        let pool_id = self.previous_literal()?;

        self.parse_parameter(TokenType::Fee0, TokenType::Literal)?;
        let fee_0 = self.previous_literal()?;

        self.parse_parameter(TokenType::Fee1, TokenType::Literal)?;
        let fee_1 = self.previous_literal()?;

        Ok(Opcode::Claim {
            pool_id,
            fee_0,
            fee_1,
        })
    }

    fn deallocate(&self) -> Result<Opcode, ()> {
        self.match_token(TokenType::Deallocate)?;
        self.match_token(TokenType::Colon)?;

        self.parse_parameter(TokenType::UseMax, TokenType::Literal)?;
        let use_max = self.previous_literal()?;

        self.parse_parameter(TokenType::PoolId, TokenType::Literal)?;
        let pool_id = self.previous_literal()?;

        self.parse_parameter(TokenType::DeltaLiquidity, TokenType::Literal)?;
        let delta_liquidity = self.previous_literal()?;

        Ok(Opcode::Deallocate {
            use_max,
            pool_id,
            delta_liquidity,
        })
    }

    fn allocate(&self) -> Result<Opcode, ()> {
        self.match_token(TokenType::Allocate)?;
        self.match_token(TokenType::Colon)?;

        self.parse_parameter(TokenType::UseMax, TokenType::Literal)?;
        let use_max = self.previous_literal()?;

        self.parse_parameter(TokenType::PoolId, TokenType::Literal)?;
        let pool_id = self.previous_literal()?;

        self.parse_parameter(TokenType::DeltaLiquidity, TokenType::Literal)?;
        let delta_liquidity = self.previous_literal()?;

        Ok(Opcode::Allocate {
            use_max,
            pool_id,
            delta_liquidity,
        })
    }
}
