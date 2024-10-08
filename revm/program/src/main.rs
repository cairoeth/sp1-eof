// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use std::u64;

use revm::primitives::Bytes;
use revm::primitives::{Bytecode, Eof, PragueSpec};
use revm_interpreter::analysis::to_analysed;
use revm_interpreter::opcode::InstructionTable;
use revm_interpreter::DummyHost;
use revm_interpreter::{Contract, Interpreter, EMPTY_SHARED_MEMORY};
use std::sync::Arc;

/// Obtained with `forge inspect MyFib deployedBytecode` in `solidity` directory.
const BYTECODE_STR: &str = "ef0001010008020002005a006c04006800008000040101000860806040526004361015e100035f80fd5f3560e01c63f9b7c7e51415e1ffee34e100346020600319360112e1002660043563ffffffff8116811415e10013602090e3000163ffffffff60405191168152f35f80fd5f80fd5f80fd63ffffffff168015e1005e6001906001916002908263ffffffff831610e10004505050e463ffffffff8085939495169116019063ffffffff8211e100196001611eef63ffffffff809394160693011690929192e0ffbe634e487b7160e01b5f52601160045260245ffd505fe4a36469706673582212206aa470811358e76b3c6d5d4535452bf698488dd50dcfa0edf9983719e744cafd6c6578706572696d656e74616cf564736f6c637827302e382e32372d646576656c6f702e323032342e382e352b636f6d6d69742e38386366363036300066";

pub fn main() {
    // Write n to public input.
    println!("cycle-tracker-start: set up input");
    let n = sp1_zkvm::io::read::<u32>();
    sp1_zkvm::io::commit(&n);

    // The call data starts with the function selector.
    let mut call_data_raw = hex::decode("f9b7c7e5").unwrap();

    // Append the padded value of n
    let mut padded_bytes = [0u8; 32];
    padded_bytes[28..32].copy_from_slice(&n.to_be_bytes());
    call_data_raw.extend(padded_bytes);
    let input = Bytes::from(call_data_raw);

    // Read the bytecode from `BYTECODE_STR` and decode it as EOF.
    let bytecode =
        Eof::decode(Bytes::copy_from_slice(&hex::decode(BYTECODE_STR).unwrap())).unwrap();

    println!("cycle-tracker-end: set up input");

    // Set up the interpreter with the input and bytecode.
    println!("cycle-tracker-start: set up runtime");
    let mut interp = Interpreter::new(
        Contract {
            input,
            bytecode: Bytecode::Eof(Arc::new(bytecode)),
            ..Default::default()
        },
        u64::MAX,
        true,
    );

    // Revm interpreter requires a host that stores information about the execution context.
    // Since we're only executing a pure function, we set up a dummy host.
    let mut host = crate::DummyHost::default();

    // Get the instruction table for the Prague (EOF) spec.
    let table: &InstructionTable<DummyHost> =
        &revm_interpreter::opcode::make_instruction_table::<DummyHost, PragueSpec>();
    println!("cycle-tracker-end: set up runtime");

    // Run the interpreter.
    println!("cycle-tracker-start: interpreter");
    let raw_out = interp.run(EMPTY_SHARED_MEMORY, table, &mut host);
    println!("cycle-tracker-end: interpreter");

    let out: Vec<u8> = raw_out.into_result_return().unwrap().output.into();
    // Commit to the output.
    sp1_zkvm::io::commit(&out);
}
