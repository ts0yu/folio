use logos::Logos;

/// Represents a token type.
/// All opcode descriptions are taken from the FVM Yellowpaper.
#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    /// This is the default opcode. It is used to represent an unknown opcode, 
    /// and is used to initialize the FVM’s state.
    #[token("unknown")]
    Unknown,

    /// This instruction is used to add liquidity to a pool.
    /// It maintains invariant pricing for each pool that is interacted with.
    #[token("allocate")]
    Allocate,

    /// This instruction is used to remove liquidity from a pool. 
    /// It maintains invariant pricing for each pool that is interacted with.
    #[token("deallocate")]
    Deallocate,

    /// Collects all the fees generated from a positive invariant for.
    #[token("claim")]
    Claim,

    /// This instruction is used to swap between the tokens.
    /// It maintains the invariant of the trading curve.
    #[token("swap")]
    Swap,

    /// This instruction is used to create a new pool.
    /// Initially, pools are not deployed with any capital, but are deployed with parameters for the CFMM as well as an initial price.
    #[token("create_pool")]
    CreatePool,

    /// This instruction is used to initialize a new pair of assets for which pools can be created.
    #[token("create_pair")]
    CreatePair,

    /// This instructions is used to jump to a different instruction in the FVM’s state via FVM’s pointer.
    #[token("jump")]
    Jump,
    
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
            if z == None {
                break;
            }
            tokens.push(Self::new(z.unwrap(), lex.slice()));
        }

        // tokens.push(Token::new(TokenType::Eof, ""));

        tokens
    }
}