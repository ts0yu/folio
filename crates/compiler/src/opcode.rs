#[derive(Debug, Clone)]
pub enum Opcode {
    Unknown,
    Allocate,
    Deallocate,
    CreatePair,
    CreatePool,
    Swap,
    Claim,
    Jump
}