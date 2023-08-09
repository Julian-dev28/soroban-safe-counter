// SPDX-License-Identifier: MIT

import "./AdditionContract.sol";

pragma solidity ^0.8.17;

/// @title SafeCounter
/// @dev This contract safely manages a counter for each address while leveraging AdditionContract for arithmetic operations.
contract SafeCounter is AdditionContract {
    /// @notice Maximum limit for each increment operation.
    uint256 public constant MAX = 10;

    /// @dev Stores the counter value for each address.
    mapping(address => uint256) private counter;

    /// @dev Event that's emitted every time a counter is incremented.
    event incrementExecuted(address indexed user, uint256 value);

    /// @dev Modifier to restrict function calls to the counter's owner only.
    modifier onlyOwnerOfCounter(address user) {
        require(msg.sender == user, "Caller is not the counter owner");
        _;
    }

    /// @notice Increment the counter for a user by a given value.
    /// @param user Address of the user for which the counter needs to be incremented.
    /// @param value Amount to increase the user's counter.
    /// @return Returns the updated counter value for the user.
    function increment(
        address user,
        uint256 value
    ) public onlyOwnerOfCounter(user) returns (uint256) {
        require(value <= MAX, "Value exceeds the allowed maximum");
        counter[user] = add(counter[user], value);
        emit incrementExecuted(user, value);
        return counter[user];
    }

    /// @notice Fetch the counter value for a user.
    /// @param user Address of the user whose counter value is to be fetched.
    /// @return The current value of the user's counter.
    function getCount(
        address user
    ) public view onlyOwnerOfCounter(user) returns (uint256) {
        return counter[user];
    }
}
