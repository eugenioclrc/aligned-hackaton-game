use std::io;

use aligned_sdk::core::types::{
    AlignedVerificationData, Network, PriceEstimate, ProvingSystemId, VerificationData,
};
use aligned_sdk::sdk::{deposit_to_aligned, estimate_fee};
use aligned_sdk::sdk::{get_next_nonce, submit_and_wait_verification};
use clap::Parser;
use dialoguer::Confirm;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, Bytes, H160, U256};
use sp1_sdk::{ProverClient, SP1Stdin};

abigen!(VerifierContract, "VerifierContract.json",);


use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
struct FinalData {
    path: String,
    pub length: u8,
    pub map: String,
}

#[derive(Serialize, Deserialize)]
struct PubInput {
    pub map: String,
    pub length: u8,
}

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SOKOBAN_ELF: &[u8] = include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    moves: String,

    #[arg(short, long)]
    keystore_path: String,
    #[arg(
        short,
        long,
        default_value = "https://ethereum-holesky-rpc.publicnode.com"
    )]
    rpc_url: String,
    #[arg(short, long, default_value = "wss://batcher.alignedlayer.com")]
    batcher_url: String,
    #[arg(short, long, default_value = "holesky")]
    network: Network,
    #[arg(short, long, default_value = "0x2A7187502059feA9f55F5b1656f5Ac2875721608")]
    verifier_contract_address: H160,
}

#[tokio::main]
async fn main() {
    // Setup the logger.
    // sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    let deserialized: FinalData = serde_json::from_str(&args.moves).unwrap();

    println!("path: {}", deserialized.path);
    println!("length: {}", deserialized.length);
    println!("map: {}", deserialized.map);

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    // moves is the serde output
    stdin.write(&args.moves);
    // Setup the prover client.
    let client = ProverClient::new();
    // Setup the program for proving.
    let (pk, vk) = client.setup(SOKOBAN_ELF);

    let rpc_url = args.rpc_url.clone();

    let keystore_password = rpassword::prompt_password("Enter keystore password: ")
        .expect("Failed to read keystore password");

    let provider =
        Provider::<Http>::try_from(rpc_url.as_str()).expect("Failed to connect to provider");

    let chain_id = provider
        .get_chainid()
        .await
        .expect("Failed to get chain_id");

    let wallet = LocalWallet::decrypt_keystore(args.keystore_path, &keystore_password)
        .expect("Failed to decrypt keystore")
        .with_chain_id(chain_id.as_u64());

    let signer = SignerMiddleware::new(provider.clone(), wallet.clone());

    if Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Do you want to deposit 0.004eth in Aligned ?\nIf you already deposited Ethereum to Aligned before, this is not needed")
        .interact()
        .expect("Failed to read user input") {   

        deposit_to_aligned(U256::from(4000000000000000u128), signer.clone(), args.network).await
        .expect("Failed to pay for proof submission");
    }

   

    // Generate the proof

    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("failed to generate proof");

    println!("Successfully generated proof!");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");


    println!("Generating Proof ");

    let client = ProverClient::new();
    let (pk, vk) = client.setup(SOKOBAN_ELF);

    
    println!("Proof generated successfully. Verifying proof...");
    client.verify(&proof, &vk).expect("verification failed");
    println!("Proof verified successfully.");

    println!("Payment successful. Submitting proof...");

    // Serialize proof into bincode (format used by sp1)
    let proof = bincode::serialize(&proof).expect("Failed to serialize proof");

    println!("Payment successful. Submitting proof...");

    let pub_input_struct = PubInput {
        map: deserialized.map,
        length: deserialized.length,
    };
    let pub_input = bincode::serialize(&pub_input_struct)
        .expect("Failed to serialize public input")
        .to_vec();

    let verification_data = VerificationData {
        proving_system: ProvingSystemId::SP1,
        proof,
        proof_generator_addr: wallet.address(),
        vm_program_code: Some(SOKOBAN_ELF.to_vec()),
        verification_key: None,
        pub_input: Some(pub_input.clone()),
    };

    let max_fee = estimate_fee(&rpc_url, PriceEstimate::Default)
        .await
        .expect("failed to fetch gas price from the blockchain");
    
    let max_fee_string = ethers::utils::format_units(max_fee, 18).unwrap();

    if !Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(format!("Aligned will use at most {max_fee_string} eth to verify your proof. Do you want to continue?"))
        .interact()
        .expect("Failed to read user input")
    {   return; }

    let nonce = get_next_nonce(&rpc_url, wallet.address(), args.network)
        .await
        .expect("Failed to get next nonce");

        println!("Submitting your proof...");

    let aligned_verification_data = submit_and_wait_verification(
        &args.batcher_url,
        &rpc_url,
        args.network,
        &verification_data,
        max_fee,
        wallet.clone(),
        nonce,
    )
    .await
    .unwrap();

    println!(
        "Proof submitted and verified successfully on batch {}",
        hex::encode(aligned_verification_data.batch_merkle_root)
    );
    println!("Claiming NFT prize...");

    claim_nft_with_verified_proof(
        &aligned_verification_data,
        signer,
        &args.verifier_contract_address,
        &pub_input.clone()
    )
    .await
    .expect("Claiming of NFT failed ...");
}

async fn claim_nft_with_verified_proof(
    aligned_verification_data: &AlignedVerificationData,
    signer: SignerMiddleware<Provider<Http>, LocalWallet>,
    verifier_contract_addr: &Address,
    pub_input: &[u8],
) -> anyhow::Result<()> {
    let verifier_contract = VerifierContract::new(*verifier_contract_addr, signer.into());

    let index_in_batch = U256::from(aligned_verification_data.index_in_batch);
    let merkle_path = Bytes::from(
        aligned_verification_data
            .batch_inclusion_proof
            .merkle_path
            .as_slice()
            .into_iter()  // Convert the slice reference into an iterator
            .flatten()    // Now flatten the iterator
            .copied()     // Copy the elements because we are working with references
            .collect::<Vec<u8>>(),
    );

    let receipt = verifier_contract
        .verify_batch_inclusion(
            aligned_verification_data
                .verification_data_commitment
                .proof_commitment,
            aligned_verification_data
                .verification_data_commitment
                .pub_input_commitment,
            aligned_verification_data
                .verification_data_commitment
                .proving_system_aux_data_commitment,
            aligned_verification_data
                .verification_data_commitment
                .proof_generator_addr,
            aligned_verification_data.batch_merkle_root,
            merkle_path,
            index_in_batch,
            Bytes::from(pub_input.to_vec()),
        )
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to send tx {}", e))?
        .await
        .map_err(|e| anyhow::anyhow!("Failed to submit tx {}", e))?;

    match receipt {
        Some(receipt) => {
            println!(
                "Prize claimed successfully. Transaction hash: {:x}",
                receipt.transaction_hash
            );
            Ok(())
        }
        None => {
            anyhow::bail!("Failed to claim prize: no receipt");
        }
    }
}