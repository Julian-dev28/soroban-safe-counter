use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct AdditionContract;

// Define a trait named 'Sum' which specifies a set of behaviors (methods) that types might have.
// In this case, any type that implements this trait will have to provide an implementation for the 'add' method.
pub(crate) trait AdditionTrait {
    fn add(a: u32, b: u32) -> u32;
}

// Implementing the 'Sum' trait for the 'AdditionContract' struct.
// This means that the 'AdditionContract' type now has a behavior as specified by the 'Sum' trait.
#[contractimpl]
impl AdditionTrait for AdditionContract {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
