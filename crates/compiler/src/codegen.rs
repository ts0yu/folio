use std::collections::HashMap;

use bytes::Bytes;

use crate::{
    assembler::{Assembler, Expression, Macro},
    opcode::Opcode,
};

use ethers::core::abi::encode_packed;

use ethabi::token::Token;

use ethereum_types::U256;
use ethereum_types::H160;

pub struct Codegen {
    opcodes: Vec<Opcode>,
}

impl<'a> Codegen {
    /// Public constructor function to instantiate a `Codegen`.
    pub fn new(exprs: Vec<Expression<'a>>) -> Self {
        let mut opcodes = Vec::new();

        for i in exprs {
            match i {
                Expression::Opcode(o) => opcodes.push(o),
                _ => panic!("this shouldnt happen"),
            }
        }

        println!("{opcodes:#?}");

        Self { opcodes }
    }

    /// Expand all macros and encode into hex, ready to be executed on the FVM.
    pub fn encode(&self) -> Bytes {
        let mut bytes = Bytes::new();
        
        for i in &self.opcodes {
            match i {
                Opcode::Allocate { use_max, pool_id, delta_liquidity } => continue,
                Opcode::Deallocate { use_max, pool_id, delta_liquidity } => continue,
                Opcode::CreatePair { token_0, token_1 } => { 
                    bytes = Bytes::from(encode_packed(
                        &[Token::Address(*token_0), 
                        Token::Address(*token_1)]
                    ).unwrap());
                },
                Opcode::CreatePool { pair_id, controller, priority_fee, fee, vol, dur, jit, max_price, price } => continue,
                Opcode::Swap { use_max, pool_id, amount_0, amount_1, sell_asset } => continue,
                Opcode::Claim { pool_id, fee_0, fee_1 } => continue,
                Opcode::Jump => continue,
                _ => continue,
            }
        }
        bytes
    }
}
