<<<<<<< Updated upstream
#![warn(missing_docs)]

use eth_encode_packed::ethabi::ethereum_types::{Address};
=======
use eth_encode_packed::ethabi::ethereum_types::Address;
>>>>>>> Stashed changes

/// Type representing an FVM opcode.
/// This is the lowest level representation folio code will be lowered to, before bytecode is generated from it.
#[derive(Debug, Clone)]
pub enum Opcode {
    /// This is the default opcode. It is used to represent an unknown opcode,
    /// and is used to initialize the FVM’s state.
    Unknown,

    /// This instruction is used to add liquidity to a pool.
    /// It maintains invariant pricing for each pool that is interacted with.    
    Allocate {
        use_max: usize,
        pool_id: usize,
        delta_liquidity: usize,
    },

    /// This instruction is used to remove liquidity from a pool.
    /// It maintains invariant pricing for each pool that is interacted with.
    Deallocate {
        use_max: usize,
        pool_id: usize,
        delta_liquidity: usize,
    },

    /// This instruction is used to initialize a new pair of assets for which pools can be created.    
    CreatePair { token_0: Address, token_1: Address },

    /// This instruction is used to create a new pool.
    /// Initially, pools are not deployed with any capital, but are deployed with parameters for the CFMM as well as an initial price.    
    CreatePool {
        pair_id: usize,
        controller: Address,
        priority_fee: usize,
        fee: usize,
        vol: usize,
        dur: usize,
        jit: usize,
        max_price: usize,
        price: usize,
    },

    /// This instruction is used to swap between the tokens.
    /// It maintains the invariant of the trading curve.    
    Swap {
        use_max: usize,
        pool_id: usize,
        amount_0: usize,
        amount_1: usize,
        sell_asset: usize,
    },

    /// Collects all the fees generated from a positive invariant for.
    Claim {
        pool_id: usize,
        fee_0: usize,
        fee_1: usize,
    },

    /// This instructions is used to jump to a different instruction in the FVM’s state via FVM’s pointer.
    Jump,
}
