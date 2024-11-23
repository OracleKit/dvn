// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Test {
    uint256 count = 0;

    function greet() external {
        count += 1;
    }

    function counter() external view returns(uint256) {
        return count;
    }
}