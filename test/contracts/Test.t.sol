// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {Test as TestContract} from "../../src/contracts/Test.sol";

contract TTestContract is Test {
    TestContract _contract;
    
    function setUp() public {
        _contract = new TestContract();
    }

    function test_test() public {
        uint256 initial_value = _contract.counter();
        _contract.greet();
        uint256 final_value = _contract.counter();

        assertEq(initial_value + 1, final_value);
    }
}