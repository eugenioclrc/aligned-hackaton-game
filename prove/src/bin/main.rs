use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin};
use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;
use base64::{Engine as _, engine::general_purpose};
use std::fs::File;
use std::io::Write;

use bincode;

#[derive(Serialize, Deserialize)]
struct FinalData {
    path: String,
    length: u8,
}

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SOKOBAN_ELF: &[u8] = include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long)]
    moves: String,
}

fn save_to_file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    // moves is the serde output
    stdin.write(&args.moves);

    let deserialized: FinalData = serde_json::from_str(&args.moves).unwrap();

    println!("path: {}", deserialized.path);
    println!("length: {}", deserialized.length);

    if args.prove {
        // Setup the program for proving.
        let (pk, vk) = client.setup(SOKOBAN_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");

        // Serialize the verification key and proof
        let vk_serialized = serde_json::to_string(&vk).expect("Failed to serialize verification key");
        let proof_serialized = serde_json::to_string(&proof).expect("Failed to serialize proof");

        // Encode the serialized data
        let vk_encoded = general_purpose::STANDARD.encode(&vk_serialized);
        let proof_encoded = general_purpose::STANDARD.encode(&proof_serialized);

        // Save verification key to file
        save_to_file("verification_key.txt", &vk_encoded)?;
        println!("Verification key saved to verification_key.txt");

        // Save proof to file
        save_to_file("proof.txt", &proof_encoded)?;
        println!("Proof saved to proof.txt");

        // Output the proof and verification key in a format suitable for further processing
        let output = serde_json::json!({
            "vk": vk_encoded,
            "proof": proof_encoded,
        });

        // Get the raw bytes of the proof
        //let proof_bytes = proof.bytes();
        let proof = bincode::serialize(&proof).expect("Failed to serialize proof");


        // Save proof to file in raw byte format
        let mut file = File::create("proof.bin")?;
        file.write_all(&proof)?;

   
        println!("Proof saved to proof.bin");

        //println!("Proof Output:");
        //println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    Ok(())
}