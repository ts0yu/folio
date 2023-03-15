use bytes::Bytes;
use ethabi::token::Token;
use ethers::core::abi::encode_packed;

use crate::{assembler::Expression, opcode::Opcode};

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

        Self { opcodes }
    }

    /// Expand all macros and encode into hex, ready to be executed on the FVM.
    pub fn encode(&self) -> Vec<Bytes> {
        let mut bytes = Vec::new();

        for i in &self.opcodes {
            match i {
                Opcode::Allocate {
                    use_max,
                    pool_id,
                    delta_liquidity,
                } => {
                    bytes.push(Bytes::from(
                        encode_packed(&[
                            Token::Int((*use_max).into()),
                            Token::Int((*pool_id).into()),
                            Token::Int((*delta_liquidity).into()),
                        ])
                        .unwrap(),
                    ));
                }
                Opcode::Deallocate {
                    use_max,
                    pool_id,
                    delta_liquidity,
                } => {
                    bytes.push(Bytes::from(
                        encode_packed(&[
                            Token::Int((*use_max).into()),
                            Token::Int((*pool_id).into()),
                            Token::Int((*delta_liquidity).into()),
                        ])
                        .unwrap(),
                    ));
                }
                Opcode::CreatePair { token_0, token_1 } => {
                    bytes.push(Bytes::from(
                        encode_packed(&[Token::Address(*token_0), Token::Address(*token_1)])
                            .unwrap(),
                    ));
                }
                Opcode::CreatePool {
                    pair_id,
                    controller,
                    priority_fee,
                    fee,
                    vol,
                    dur,
                    jit,
                    max_price,
                    price,
                } => {
                    bytes.push(Bytes::from(
                        encode_packed(&[
                            Token::Int((*pair_id).into()),
                            Token::Address(*controller),
                            Token::Int((*priority_fee).into()),
                            Token::Int((*fee).into()),
                            Token::Int((*vol).into()),
                            Token::Int((*dur).into()),
                            Token::Int((*jit).into()),
                            Token::Int((*max_price).into()),
                            Token::Int((*price).into()),
                        ])
                        .unwrap(),
                    ));
                }
                Opcode::Swap {
                    use_max,
                    pool_id,
                    amount_0,
                    amount_1,
                    sell_asset,
                } => {
                    bytes.push(Bytes::from(
                        encode_packed(&[
                            Token::Int((*use_max).into()),
                            Token::Int((*pool_id).into()),
                            Token::Int((*amount_0).into()),
                            Token::Int((*amount_1).into()),
                            Token::Int((*sell_asset).into()),
                        ])
                        .unwrap(),
                    ));
                }
                Opcode::Claim {
                    pool_id,
                    fee_0,
                    fee_1,
                } => {
                    bytes.push(Bytes::from(
                        encode_packed(&[
                            Token::Int((*pool_id).into()),
                            Token::Int((*fee_0).into()),
                            Token::Int((*fee_1).into()),
                        ])
                        .unwrap(),
                    ));
                }
                Opcode::Jump => continue,
                _ => continue,
            }
        }
        bytes
    }
}
