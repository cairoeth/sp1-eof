// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use std::u64;

use revm::primitives::Bytes;
use revm::primitives::{Bytecode, CancunSpec};
use revm_interpreter::analysis::to_analysed;
use revm_interpreter::opcode::InstructionTable;
use revm_interpreter::DummyHost;
use revm_interpreter::{Contract, Interpreter, EMPTY_SHARED_MEMORY};

/// The bytecode we want to execute inside the EVM.
/// This is obtained by running vyper src/Fib.vy --experimental-codegen
const BYTECODE_STR: &str = "6100a861000f6000396100a86000f35f3560e01c63f9b7c7e5186100a0573660241134176100a4576004358060201c6100a4578061003557505f60605260605b602090f35b60019060019080600212610053575090505b61010052610100610030565b806002116100a45760029060029003806103e8106100a4576002019290915b82841861008457905090509050610047565b809101908160201c6100a45790611eef90069160010191610072565b5f5ffd5b5f80fd85582021e688f88ccf98d563cd8f3ba0cfde699c9bce6d1aaad71cf942e4c2af09847418a88000a1657679706572830004010034";

pub fn main() {
    // Write n to public input.
    println!("cycle-tracker-start: set up input");
    let n = sp1_zkvm::io::read::<u32>();
    sp1_zkvm::io::commit(&n);

    // First, we need to format the call data.
    // 
    // The call data starts with the function selector.
    let mut call_data_raw = hex::decode("f9b7c7e5").unwrap();

    // Then, we append the padded value of n
    let mut padded_bytes = [0u8; 32];
    padded_bytes[28..32].copy_from_slice(&n.to_be_bytes());
    call_data_raw.extend(padded_bytes);
    let input = Bytes::from(call_data_raw);

    // We also need to read the bytecode from `BYTECODE_STR`.
    let bytecode = to_analysed(
        Bytecode::new_raw_checked(Bytes::copy_from_slice(&hex::decode(BYTECODE_STR).unwrap()))
            .unwrap(),
    );
    println!("cycle-tracker-end: set up input");

    // To set up the interpreter, we first instantiate it with the input and bytecode.
    println!("cycle-tracker-start: set up runtime");
    let mut interp = Interpreter::new(
        Contract {
            input,
            bytecode,
            ..Default::default()
        },
        u64::MAX,
        true,
    );

    // The Revm interpreter requires a host that stores information about the execution context.
    // Since we're only executing a pure function, we set up a dummy host.
    let mut host = crate::DummyHost::default();
    
    // We get an instruction table from the Cancun Spec.
    let table: &InstructionTable<DummyHost> =
        &revm_interpreter::opcode::make_instruction_table::<DummyHost, CancunSpec>();
    println!("cycle-tracker-end: set up runtime");

    // Finally, we run the interpreter.
    println!("cycle-tracker-start: interpreter");
    let raw_out = interp.run(EMPTY_SHARED_MEMORY, table, &mut host);
    println!("cycle-tracker-end: interpreter");

    let out: Vec<u8> = raw_out.into_result_return().unwrap().output.into();
    // Commit to the output.
    sp1_zkvm::io::commit(&out);
}
