use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct AdditionContract;

// This could be in the same file or a different module.
pub(crate) trait Sum {
    fn add(a: u32, b: u32) -> u32;
}

#[contractimpl]
impl Sum for AdditionContract {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
