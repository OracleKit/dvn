// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

contract TestContract is Test {
    function test_mock() public {
        uint a = 100;
        assertEq(a, 100);
    }
}