//! This contract demonstrates how to implement authorization using
//! Soroban-managed auth framework for a simple case (a single user that needs
//! to authorize a single contract invocation).
//!
//! See `timelock` and `single_offer` examples for demonstration of performing
//! authorized token operations on behalf of the user.
//!
//! See `atomic_swap` and `atomic_multiswap` examples for demonstration of
//! multi-party authorizaton.
//!
//! See `account` example for demonstration of an acount contract with
//! a custom authentication scheme and a custom authorization policy.
#![no_std]

// Import the required functionality from the `addition_contract` module.
// Here we're bringing in the `AdditionContract` structure and the `Sum` trait.
use crate::addition_contract::{AdditionContract, Sum};

// Macro:
// Definition: A reusable chunk of code expanded at compile-time.
// Soroban SDK: Used for tasks like generating specific data types.

// Attribute Macro:
// Definition: A macro applied to items using #[macro_name], modifying their behavior.
// Soroban SDK: Marks types or functions for special handling, like contract. Signals to the SDK how to handle them or the special behavior they should exhibit.

// Struct:
// Definition: A custom data type grouping different values together.
// Soroban SDK: Represents elements like contract identifiers (Address) or the execution environment (Env).

// Import the necessary definitions from the `soroban_sdk` crate.
use soroban_sdk::{
    contract, // (attribute macro) Marks a type as being the type that contract functions are attached for.
    contracterror, // (attribute macro) Generates conversions from the repr(u32) enum from/into an Error.
    contractimpl, // (attribute macro) Exports the publicly accessible functions to the Soroban environment.
    contracttype, // (attribute macro) Generates conversions from the struct/enum from/into a Val.
    symbol_short, // (macro) Create a short Symbol constant with the given string.
    Address,      // (struct) Address is a universal opaque identifier to use in contracts.
    Env,          // (struct) Provides access to the environment the contract is executing within.
};

// Define custom error types specific to the operations of this contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    MaxValueExceeded = 0, // Error to indicate an attempt to exceed the allowed maximum value.
}

// Define the upper limit for the counter.
const MAX: u32 = 10;

// Tuple enum is used to define a key for the data being stored.
#[contracttype]
enum DataKey {
    Counter(Address), // The key corresponding to a counter associated with an address.
                      // History(Address), // The key corresponding to a history associated with an address.
}

// Primary contract structure representing a counter with controlled increment capability.
#[contract]
pub struct SafeCounter;

// Logic implementation of the SafeCounter contract.
#[contractimpl]
impl SafeCounter {
    /// Increment increments a counter for the user, and returns the value.
    pub fn increment(env: Env, user: Address, value: u32) -> Result<u32, Error> {
        // Requires `user` to have authorized call of the `increment` of this
        // contract with all the arguments passed to `increment`, i.e. `user`
        // and `value`. This will panic if auth fails for any reason.
        // When this is called, Soroban host performs the necessary
        // authentication, manages replay prevention and enforces the user's
        // authorization policies.
        // The contracts normally shouldn't worry about these details and just
        // write code in generic fashion using `Address` and `require_auth` (or
        // `require_auth_for_args`).
        user.require_auth();

        // This call is equilvalent to the above:
        // user.require_auth_for_args((&user, value).into_val(&env));

        // The following has less arguments but is equivalent in authorization
        // scope to the above calls (the user address doesn't have to be
        // included in args as it's guaranteed to be authenticated).
        // user.require_auth_for_args((value,).into_val(&env));

        if value <= MAX {
            // Construct a key for the data being stored. Use an enum to set the
            // contract up well for adding other types of data to be stored.
            let key = DataKey::Counter(user.clone());

            // Get the current count for the invoker.
            let mut count: u32 = env.storage().persistent().get(&key).unwrap_or(0);

            // Increment the count using the `Adder` trait.
            count = AdditionContract::add(count, value);

            // Save the count.
            env.storage().persistent().set(&key, &count);

            // Publish the count to the caller.
            env.events()
                .publish((symbol_short!("increment"), user.clone(), value), count);

            Ok(count)
        } else {
            Err(Error::MaxValueExceeded)
        }

        // Return the count to the caller.
    }
    pub fn get_count(env: Env, user: Address) -> u32 {
        // Requires `user` to have authorized call of the `get` of this
        user.require_auth();

        // Construct a key for the data being stored. Use an enum to set the
        let key = DataKey::Counter(user.clone());

        // Get the current count for the invoker.
        let count: u32 = env.storage().persistent().get(&key).unwrap_or_default();

        // Return the count to the caller.
        count
    }
    pub fn get_max() -> u32 {
        // Return the MAX value to the caller.
        MAX
    }
}

mod addition_contract;
mod test;
