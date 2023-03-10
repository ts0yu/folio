use ethers::types::H160;

#[derive(Debug, Clone)]
pub enum Opcode {
    /// This is the default opcode. It is used to represent an unknown opcode,
    /// and is used to initialize the FVM’s state.
    Unknown,

    /// This instruction is used to add liquidity to a pool.
    /// It maintains invariant pricing for each pool that is interacted with.    
    Allocate {
        useMax: usize,
        poolId: usize,
        deltaLiquidity: usize,
    },

    /// This instruction is used to remove liquidity from a pool.
    /// It maintains invariant pricing for each pool that is interacted with.
    Deallocate {
        useMax: usize,
        poolId: usize,
        deltaLiquidity: usize,
    },

    /// This instruction is used to initialize a new pair of assets for which pools can be created.    
    CreatePair { token0: H160, token1: H160 },

    /// This instruction is used to create a new pool.
    /// Initially, pools are not deployed with any capital, but are deployed with parameters for the CFMM as well as an initial price.    
    CreatePool {
        pairId: usize,
        controller: H160,
        priorityFee: usize,
        fee: usize,
        vol: usize,
        dur: usize,
        jit: usize,
        maxPrice: usize,
        price: usize,
    },

    /// This instruction is used to swap between the tokens.
    /// It maintains the invariant of the trading curve.    
    Swap {
        useMax: usize,
        poolId: usize,
        amount0: usize,
        amount1: usize,
        sellAsset: usize,
    },

    /// Collects all the fees generated from a positive invariant for.
    Claim {
        poolId: usize,
        fee0: usize,
        fee1: usize,
    },

    /// This instructions is used to jump to a different instruction in the FVM’s state via FVM’s pointer.
    Jump,
}
