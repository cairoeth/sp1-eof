// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

contract MyFib {
    function fib(uint32 n) external pure returns (uint32 b) {
        if (n == 0) {
            return 0;
        }
        uint32 a = 1;
        b = 1;
        for (uint32 i = 2; i < n; i++) {
            uint32 c = (a + b) % 7919;
            a = b;
            b = c;
        }
        return b;
    }
}
