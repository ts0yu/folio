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

    fn from_amount(amount: usize) -> (usize, usize) {
        if amount == 0 { return (0, 0) };

        let mut base = amount;
        let mut power = 0;

        while base % 10 == 0 {
            power += 1;
            base /= 10;
        }

        (power, base)
    }

    fn pack(upper: usize, lower: usize) -> usize {
        (upper << 4) | lower
    }

    /// Expand all macros and encode into hex, ready to be executed on the FVM.
    pub fn encode(&self) -> Vec<Bytes> {
        let mut bytes = Vec::new();

        for i in &self.opcodes {
            match i {
                ///todo
                Opcode::Allocate {
                    use_max,
                    pool_id,
                    delta_liquidity,
                } => {
                    let (power, base) = Codegen::from_amount(*delta_liquidity);

                    bytes.push(Bytes::from(
                        encode_packed(&[
                            Token::Int(Codegen::pack(*use_max, 1).into()),
                            Token::Int((*pool_id).into()),
                            Token::Int(power.into()),
                            Token::Int(base.into()),
                        ])
                        .unwrap(),
                    ));
                }
                ///todo
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
                ///FINISHED
                Opcode::CreatePair { token_0, token_1 } => {
                    let create_pair = 12;

                    bytes.push(Bytes::from(
                        encode_packed(&[Token::Int(create_pair.into()), Token::Address(*token_0), Token::Address(*token_1)])
                            .unwrap(),
                    ));
                }
                ///FINISHED
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
                    let create_pool = 10;
                    let pointer = 36 + 16;

                    let (power0, base0) = Codegen::from_amount(*max_price);
                    let (power1, base1) = Codegen::from_amount(*price);

                    bytes.push(Bytes::from(
                        encode_packed(&[
                            Token::Int(create_pool.into()),
                            Token::Int((*pair_id).into()),
                            Token::Address(*controller),
                            Token::Int((*priority_fee).into()),
                            Token::Int((*fee).into()),
                            Token::Int((*vol).into()),
                            Token::Int((*dur).into()),
                            Token::Int((*jit).into()),
                            Token::Int((*jit).into()),
                            Token::Int((*power0).into()),
                            Token::Int((*base0).into()),
                            Token::Int((*power1).into()),
                            Token::Int((*base1).into()),
                        ])
                        .unwrap(),
                    ));
                }
                ///TODO
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
                ///FINISHED
                Opcode::Claim {
                    pool_id,
                    fee_0,
                    fee_1,
                } => {
                    bytes.push(Bytes::from(
                        let claim = 4;
                        let pointer = 8;

                        let (power_fee0, base_fee0) = Codegen::from_amount(*fee_0);
                        let (power_fee1, base_fee1) = Codegen::from_amount(*fee_1);

                        encode_packed(&[
                            Token::Int(claim.into()),
                            Token::Int((*pool_id).into()),
                            Token::Int(pointer.into()),
                            Token::Int((*power_fee0).into()),
                            Token::Int((*base_fee0).into()),
                            Token::Int((*power_fee1).into()),
                            Token::Int((*base_fee2).into()),
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
