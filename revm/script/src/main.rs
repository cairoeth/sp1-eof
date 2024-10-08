use sp1_sdk::{utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use std::env;

fn main() {
    // The elf we want to execute inside the zkVM.
    let elf: &[u8] = if env::var("EOF").unwrap_or_default() == "true" {
        include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf")
    } else {
        include_bytes!("../../program_legacy/elf/riscv32im-succinct-zkvm-elf")
    };

    // Setup logging.
    utils::setup_logger();

    // Create an input stream and write '1000' to it.
    let n = 1000u32;

    // The input stream that the program will read from using `sp1_zkvm::io::read`. Note that the
    // types of the elements in the input stream must match the types being read in the program.
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Create a `ProverClient` method.
    let client = ProverClient::new();

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(elf, stdin.clone()).run().unwrap();
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    // Generate the proof for the given program and input.
    let (pk, vk) = client.setup(elf);
    let mut proof = client.prove(&pk, stdin).run().unwrap();

    println!("generated proof");

    // Read and verify the output.
    //
    // Note that this output is read from values commited to in the program using
    // `sp1_zkvm::io::commit`.
    let _ = proof.public_values.read::<u32>();
    let a: Vec<u8> = proof.public_values.read::<Vec<u8>>();

    println!("a: {:?}", a);

    // Verify proof and public values
    client.verify(&proof, &vk).expect("verification failed");

    // Test a round trip of proof serialization and deserialization.
    proof
        .save("proof-with-pis.bin")
        .expect("saving proof failed");
    let deserialized_proof =
        SP1ProofWithPublicValues::load("proof-with-pis.bin").expect("loading proof failed");

    // Verify the deserialized proof.
    client
        .verify(&deserialized_proof, &vk)
        .expect("verification failed");

    println!("successfully generated and verified proof for the program!")
}
