// SPDX-License-Identifier: MIT
import "./AdditionContract.sol";

pragma solidity ^0.8.17;

contract SafeCounter is AdditionContract {
    uint256 public constant MAX = 10;

    mapping(address => uint256) private counter;

    event incrementExecuted(address indexed user, uint256 value);

    modifier onlyOwnerOfCounter(address user) {
        require(msg.sender == user, "Caller is not the counter owner");
        _;
    }

    function increment(
        address user,
        uint256 value
    ) public onlyOwnerOfCounter(user) returns (uint256) {
        require(value <= MAX, "Value exceeds the allowed maximum");

        counter[user] = add(counter[user], value);

        emit incrementExecuted(user, value);

        return counter[user];
    }

    function getCount(
        address user
    ) public view onlyOwnerOfCounter(user) returns (uint256) {
        return counter[user];
    }
}
