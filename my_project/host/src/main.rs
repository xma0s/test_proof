use json_core::Outputs;
use json_methods::SEARCH_JSON_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    // JSON input
    let data = r#"
    {
        "msg": "This is a confidential message.",
        "user_address": "0x123456789abcdef",
        "history_uiid": "abcde-12345-67890",
        "is_shilling_allowed": true
    }"#;

    let outputs = run_zkvm(data);

    println!();
    println!("  {:?}", outputs.hash);
    println!(
        "provably contains a field 'is_shilling_allowed' with value {}",
        outputs.data
    );
}

/// Runs the zkVM and returns the verified output
fn run_zkvm(data: &str) -> Outputs {
    let env = ExecutorEnv::builder()
        .write(&data)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, SEARCH_JSON_ELF).unwrap().receipt;

    // Verify proof
    receipt.verify(SEARCH_JSON_ELF).unwrap();

    receipt.journal.decode().unwrap()
}
