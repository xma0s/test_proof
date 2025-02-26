#![no_main]

use json::parse;
use json_core::Outputs;
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

risc0_zkvm::guest::entry!(main);

fn main() {
    // Read JSON input from host
    let data: String = env::read();
    
    // Compute SHA256 hash of full data (used for verification)
    let data_hash = *Impl::hash_bytes(&data.as_bytes());

    // Parse JSON input
    let parsed_data = parse(&data).unwrap();

    // Extract values
    let msg = parsed_data["msg"].as_str().unwrap();
    let is_shilling_allowed = parsed_data["is_shilling_allowed"].as_bool().unwrap();
    
    // Hash sensitive data (user_address & history_uiid)
    let mut hasher = Sha256::new();
    hasher.update(parsed_data["user_address"].as_str().unwrap().as_bytes());
    hasher.update(parsed_data["history_uiid"].as_str().unwrap().as_bytes());
    let hidden_hash = hasher.finalize();

    // Output data without exposing sensitive information
    let output = Outputs {
        data: is_shilling_allowed as u32, // Convert bool to u32
        hash: data_hash, // Hash of entire JSON input for proof verification
    };

    // Commit the output to zkVM
    env::commit(&output);
}
