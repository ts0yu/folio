use eth_encode_packed::ethabi::ethereum_types::{Address, U256};

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
        use_max: U256,
        pool_id: U256,
        delta_liquidity: U256,
    },

    /// This instruction is used to remove liquidity from a pool.
    /// It maintains invariant pricing for each pool that is interacted with.
    Deallocate {
        use_max: U256,
        pool_id: U256,
        delta_liquidity: U256,
    },

    /// This instruction is used to initialize a new pair of assets for which pools can be created.    
    CreatePair { token_0: Address, token_1: Address },

    /// This instruction is used to create a new pool.
    /// Initially, pools are not deployed with any capital, but are deployed with parameters for the CFMM as well as an initial price.    
    CreatePool {
        pair_id: U256,
        controller: Address,
        priority_fee: U256,
        fee: U256,
        vol: U256,
        dur: U256,
        jit: U256,
        max_price: U256,
        price: U256,
    },

    /// This instruction is used to swap between the tokens.
    /// It maintains the invariant of the trading curve.    
    Swap {
        use_max: U256,
        pool_id: U256,
        amount_0: U256,
        amount_1: U256,
        sell_asset: U256,
    },

    /// Collects all the fees generated from a positive invariant for.
    Claim {
        pool_id: U256,
        fee_0: U256,
        fee_1: U256,
    },

    /// This instructions is used to jump to a different instruction in the FVM’s state via FVM’s pointer.
    Jump,
}
