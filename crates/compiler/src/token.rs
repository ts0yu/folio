use logos::Logos;
use crate::diagnostics::Diagnostics;

/// Represents a token type.
/// All opcode descriptions are taken from the FVM Yellowpaper.
#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    #[token("unknown")]
    Unknown,

    #[token("allocate")]
    Allocate,

    #[token("deallocate")]
    Deallocate,

    #[token("claim")]
    Claim,

    #[token("swap")]
    Swap,

    #[token("createPool")]
    CreatePool,

    #[token("createPair")]
    CreatePair,

    #[token("jump")]
    Jump,

    #[token("macro")]
    Macro,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token(":")]
    Colon,

    #[token("poolId")]
    PoolId,

    #[token("fee0")]
    Fee0,

    #[token("fee1")]
    Fee1,

    #[token("useMax")]
    UseMax,

    #[token("deltaLiquidity")]
    DeltaLiquidity,

    #[token("amount0")]
    Amount0,

    #[token("amount1")]
    Amount1,

    #[token("token0")]
    Token0,

    #[token("token1")]
    Token1,

    #[token("pairId")]
    PairId,

    #[token("controller")]
    Controller,

    #[token("priorityFee")]
    PriorityFee,

    #[token("fee")]
    Fee,

    #[token("vol")]
    Vol,

    #[token("dur")]
    Dur,

    #[token("jit")]
    Jit,

    #[token("maxPrice")]
    MaxPrice,

    #[token("price")]
    Price,

    #[token("sellAsset")]
    SellAsset,

    #[regex(r"0[xX][a-fA-F0-9]+")]
    AddressLiteral,

    #[regex("[a-zA-Z_]+")]
    Identifier,

    #[regex("[+-]?([0-9]*[.])?[0-9]+")]
    Literal,

    /// Unrecognized or erroneous token.
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

/// Represents a token.
/// Should include a span later on as well.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub slice: &'a str,
}

impl<'a> Token<'a> {
    /// Instantiate a token.
    pub fn new(ttype: TokenType, slice: &'a str) -> Self {
        Self { ttype, slice }
    }

    /// Given a source chunk, lex it and provide a vector of tokens.
    pub fn lex(raw: &'a str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut lex = TokenType::lexer(raw);

        loop {
            let z = lex.next();
            if z.is_none() {
                break;
            }
            if z.unwrap() == TokenType::Error {
                Diagnostics::emit(
                    String::from(raw),
                    String::from("example.fvm"),
                    lex.span().start as u64,
                    lex.span().end as u64,
                    "invalid token".to_string(),
                    "invalid".to_string(),
                    "E000".to_string(),
                );
                break;
            } else {
                tokens.push(Self::new(z.unwrap(), lex.slice()))
            };
        }

        // println!("{tokens:#?}");

        tokens
    }
}
