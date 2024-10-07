// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

import "forge-std/Test.sol";

import {MyFib} from "src/Fib.sol";

contract TestFib is Test {
    MyFib fib;

    function setUp() public {
        fib = new MyFib();
    }

    function testFib() public {
        assertEq(fib.fib(0), 0);
        assertEq(fib.fib(1), 1);
        assertEq(fib.fib(2), 1);
        assertEq(fib.fib(3), 2);
        assertEq(fib.fib(4), 3);
        assertEq(fib.fib(5), 5);
        assertEq(fib.fib(6), 8);
    }

    // Exclude from coverage report
    function test() public {}
}
