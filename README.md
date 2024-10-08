# SP1 performance with EOF 🏎️

EVM Object Format (EOF) is an upgrade to the Ethereum Virtual Machine (EVM) that introduces an extensible and versioned container format for EVM bytecode with a once-off validation at deploy time, focusing on separating code and data, enhancing code validation, and improving overall efficiency.

This repository benchmarks the performance of the Revm EOF and legacy EVM interpreters in SP1. 

## Results

The following table shows the performance of the EOF-based and legacy EVM interpreters for the Fibonacci contract. As can be observed, the EOF-based program is 2.9 times more efficient for the total and interpreter cycles (65.80% fewer cycles required) and gas. It also runs 2.69 times faster and has a proof size 2.04 times smaller.

| Program    | Set up input (cycles) | Set up runtime (cycles) | Interpreter (cycles) | Total (cycles) | E2E time (s) | kHz   | Proof size |
|------------|----------------------:|------------------------:|---------------------:|---------------:|-------------:|------:|-----------:|
| EOF        |                23,276 |                  11,193 |            3,112,476 |      3,158,346 |        58.86 | 53.66 |  8,087,802 |
| Legacy EVM |                42,709 |                  11,188 |            9,101,063 |      9,166,261 |       158.47 | 57.84 | 16,489,434 |

## Running

To run the benchmarks, go to the `revm/script` directory and run:

- EOF: `RUST_LOG=info EOF=true cargo run --release`
- Legacy EVM: `RUST_LOG=info EOF=false cargo run --release`

The setup costs are roughly fixed, but the "run interpreter" section is responsible for executing the actual instructions of the Revm programs. The EOF bytecode of the contract can be obtained by running `forge inspect MyFib deployedBytecode` in the `solidity` directory.
