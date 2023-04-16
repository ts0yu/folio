<<<<<<< Updated upstream
#![warn(missing_docs)]

use bytes::Bytes;
use eth_encode_packed::{
    abi,
    ethabi::ethereum_types::{Address, U256},
    hex, SolidityDataType, TakeLastXBytes,
};
=======
use eth_encode_packed::{abi, ethabi::ethereum_types::U256, SolidityDataType, TakeLastXBytes};
>>>>>>> Stashed changes

use crate::{assembler::Expression, opcode::Opcode};

/// # Codegen
///
/// The bytecode generation module of folio.
/// [`Codegen`] takes in a vector of Expressions and returns a hex string, which can be executed on the FVM.

/// Code generation struct.
pub struct Codegen {
    /// Set of opcodes to generate bytecode from.
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
        if amount == 0 {
            return (0, 0);
        };

        let mut base = amount;
        let mut power = 0;

        while base % 10 == 0 {
            power += 1;
            base /= 10;
        }

        (power, base)
    }

    pub fn generate(instructions: Vec<String>) -> String {
        let mut payload = "AA".to_string() + &instructions.len().to_string();

        for i in instructions {
            let edited = i.len().to_string() + &i;

            payload += &edited;
        }

        payload
    }

    fn pack(upper: usize, lower: usize) -> usize {
        (upper << 4) | lower
    }

    /// Expand all macros and encode into hex, ready to be executed on the FVM.
    pub fn encode(&self) -> Vec<String> {
        let mut bytes = Vec::new();

        for i in &self.opcodes {
            match i {
                Opcode::Allocate {
                    use_max,
                    pool_id,
                    delta_liquidity,
                } => {
                    let (power, base) = Codegen::from_amount(*delta_liquidity);
                    let allocate = 1;
                    let packed = Codegen::pack((*use_max as u8).into(), (allocate as u8).into());

                    let (encoded, hash) = abi::encode_packed(&[
                        SolidityDataType::NumberWithShift(U256::from(packed), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(*pool_id), TakeLastXBytes(64)),
                        SolidityDataType::NumberWithShift(U256::from(power), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(base), TakeLastXBytes(128)),
                    ]);

                    bytes.push(hash)
                }
                Opcode::Deallocate {
                    use_max,
                    pool_id,
                    delta_liquidity,
                } => {
                    let (power, base) = Codegen::from_amount(*delta_liquidity);
                    let deallocate = 3;
                    let packed = Codegen::pack((*use_max as u8).into(), (deallocate as u8).into());

                    let (encoded, hash) = abi::encode_packed(&[
                        SolidityDataType::NumberWithShift(U256::from(packed), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(*pool_id), TakeLastXBytes(64)),
                        SolidityDataType::NumberWithShift(U256::from(power), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(base), TakeLastXBytes(128)),
                    ]);

                    bytes.push(hash)
                }
                Opcode::CreatePair { token_0, token_1 } => {
                    let create_pair = 12;

                    let (encoded, hash) = abi::encode_packed(&[
                        SolidityDataType::NumberWithShift(
                            U256::from(create_pair),
                            TakeLastXBytes(8),
                        ),
                        SolidityDataType::Address(*token_0),
                        SolidityDataType::Address(*token_1),
                    ]);

                    bytes.push(hash);
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
                    let create_pool = 11;
                    let pointer = 36 + 16;

                    let (power0, base0) = Codegen::from_amount(*max_price);
                    let (power1, base1) = Codegen::from_amount(*price);

                    let (encoded, hash) = abi::encode_packed(&[
                        SolidityDataType::NumberWithShift(
                            U256::from(create_pool),
                            TakeLastXBytes(8),
                        ),
                        SolidityDataType::NumberWithShift(U256::from(*pair_id), TakeLastXBytes(24)),
                        SolidityDataType::Address(*controller),
                        SolidityDataType::NumberWithShift(
                            U256::from(*priority_fee),
                            TakeLastXBytes(16),
                        ),
                        SolidityDataType::NumberWithShift(U256::from(*fee), TakeLastXBytes(16)),
                        SolidityDataType::NumberWithShift(U256::from(*vol), TakeLastXBytes(16)),
                        SolidityDataType::NumberWithShift(U256::from(*dur), TakeLastXBytes(16)),
                        SolidityDataType::NumberWithShift(U256::from(*jit), TakeLastXBytes(16)),
                        SolidityDataType::NumberWithShift(U256::from(pointer), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(power0), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(base0), TakeLastXBytes(128)),
                        SolidityDataType::NumberWithShift(U256::from(power1), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(base1), TakeLastXBytes(128)),
                    ]);

                    bytes.push(hash)
                }
                Opcode::Swap {
                    use_max,
                    pool_id,
                    amount_0,
                    amount_1,
                    sell_asset,
                } => {
                    let swap = if *sell_asset == 1 {
                        6
                    } else {
                        5
                    };

                    let (power0, base0) = Codegen::from_amount(*amount_0);
                    let (power1, base1) = Codegen::from_amount(*amount_1);

                    let packed = Codegen::pack((*use_max as u8).into(), (swap as u8).into());

                    let (encoded, hash) = abi::encode_packed(&[
                        SolidityDataType::NumberWithShift(U256::from(packed), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(*pool_id), TakeLastXBytes(64)),
                        SolidityDataType::NumberWithShift(U256::from(27), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(power0), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(base0), TakeLastXBytes(128)),
                        SolidityDataType::NumberWithShift(U256::from(power1), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(base1), TakeLastXBytes(128)),
                    ]);

                    bytes.push(hash)
                }
                Opcode::Claim {
                    pool_id,
                    fee_0,
                    fee_1,
                } => {
                    let claim = 4;
                    let (power_fee0, base_fee0) = Codegen::from_amount(*fee_0);
                    let (power_fee1, base_fee1) = Codegen::from_amount(*fee_1);

                    let (encoded, hash) = abi::encode_packed(&[
                        SolidityDataType::NumberWithShift(U256::from(claim), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(U256::from(*pool_id), TakeLastXBytes(64)),
                        SolidityDataType::NumberWithShift(U256::from(27), TakeLastXBytes(8)),
                        SolidityDataType::NumberWithShift(
                            U256::from(power_fee0),
                            TakeLastXBytes(8),
                        ),
                        SolidityDataType::NumberWithShift(
                            U256::from(base_fee0),
                            TakeLastXBytes(128),
                        ),
                        SolidityDataType::NumberWithShift(
                            U256::from(power_fee1),
                            TakeLastXBytes(8),
                        ),
                        SolidityDataType::NumberWithShift(
                            U256::from(base_fee1),
                            TakeLastXBytes(128),
                        ),
                    ]);
                    bytes.push(hash);
                }
                Opcode::Jump => continue,
                _ => continue,
            }
        }
        bytes
    }
}
