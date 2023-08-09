// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

interface AdditionInterface {
    function add(uint256 a, uint256 b) external pure returns (uint256 sum);
}

contract AdditionContract is AdditionInterface {
    function add(
        uint256 a,
        uint256 b
    ) public pure override returns (uint256 sum) {
        sum = a + b;
    }
}
