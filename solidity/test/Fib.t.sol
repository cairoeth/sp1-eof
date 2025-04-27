// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

import "forge-std/Test.sol";

import {MyFib} from "src/Fib.sol";

contract TestFib is Test {
    MyFib fib;
    MyFib vyFib;

    function setUp() public {
        fib = new MyFib();
        vyFib = MyFib(deployCode("Fib"));

        bytes memory solFib = vm.getDeployedCode("Fib.sol:MyFib");
        bytes memory vyFib = vm.getDeployedCode("Fib.vy");

        console.logBytes(solFib);
        console.logBytes(vyFib);
    }

    function testFib() public {
        assertEq(fib.fib(0), 0);
        assertEq(fib.fib(1), 1);
        assertEq(fib.fib(2), 1);
        assertEq(fib.fib(3), 2);
        assertEq(fib.fib(4), 3);
        assertEq(fib.fib(5), 5);
        assertEq(fib.fib(6), 8);
        assertEq(fib.fib(1000), 5965);
    }

    function testVyFib() public {
        assertEq(vyFib.fib(0), 0);
        assertEq(vyFib.fib(1), 1);
        assertEq(vyFib.fib(2), 1);
        assertEq(vyFib.fib(3), 2);
        assertEq(vyFib.fib(4), 3);
        assertEq(vyFib.fib(5), 5);
        assertEq(vyFib.fib(6), 8);
        assertEq(vyFib.fib(1000), 5965);
    }

    // Exclude from coverage report
    function test() public {}
}
