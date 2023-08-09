// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

/// @title Addition Interface
/// @dev Interface defining the arithmetic addition function.
interface AdditionInterface {
    /// @notice Computes the sum of two unsigned integers.
    /// @param a First integer.
    /// @param b Second integer.
    /// @return sum The sum of the two integers.
    function add(uint256 a, uint256 b) external pure returns (uint256 sum);
}

/// @title Addition Contract
/// @dev A contract that implements the `AdditionInterface`.
contract AdditionContract is AdditionInterface {
    /// @notice Add two given unsigned integers.
    /// @dev Implements the addition function from the `AdditionInterface`.
    /// @param a First unsigned integer.
    /// @param b Second unsigned integer.
    /// @return sum Returns the sum of `a` and `b`.
    function add(uint256 a, uint256 b) public pure returns (uint256 sum) {
        sum = a + b; // Compute the sum of `a` and `b`.
    }
}
