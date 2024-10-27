use aligned_sdk::core::types::{
    AlignedVerificationData, Network, PriceEstimate, ProvingSystemId, VerificationData,
};
use aligned_sdk::sdk::{deposit_to_aligned, estimate_fee};
use aligned_sdk::sdk::{get_next_nonce, submit_and_wait_verification};
use clap::Parser;
use dialoguer::Confirm;
use ethers::abi::{encode, Token};
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, Bytes, H160, U256};

use sp1_sdk::{ProverClient, SP1Stdin};

// somehow this not work
// use sokoban::{decode_moves, string_to_bytes, Game, Level};

abigen!(VerifierContract, "VerifierContract.json",);

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct FinalData {
    path: String,
    length: u128, // length is the amount of moves to solve the puzzle
    rows: u32,
    cols: u32,
    player_col: u32,
    player_row: u32,
    map: String,
}

#[derive(Serialize, Deserialize)]
struct PubInput {
    length: u128, // length is the amount of moves to solve the puzzle
    rows: u32,
    cols: u32,
    player_col: u32,
    player_row: u32,
    map: Vec<u8>,
}
impl PubInput {
    pub fn encode_to_bytes(&self) -> Bytes {
        // Convert struct fields into ABI-encodable tokens
        let tokens = vec![
            Token::Uint(self.length.into()),
            Token::Uint(self.rows.into()),
            Token::Uint(self.cols.into()),
            Token::Uint(self.player_col.into()),
            Token::Uint(self.player_row.into()),
            Token::Bytes(self.map.clone()),
        ];

        // Encode the tokens into bytes
        Bytes::from(encode(&tokens))
    }

    // Helper function if you need the raw Vec<u8>
    pub fn encode_to_vec(&self) -> Vec<u8> {
        self.encode_to_bytes().to_vec()
    }
}

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SOKOBAN_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    data: String,

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
    #[arg(
        short,
        long,
        default_value = "0x37E84746A4631f80d279bcB410dd320f57C1B842"
    )]
    verifier_contract_address: H160,
}

#[tokio::main]
async fn main() {
    // Setup the logger.
    // sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    let deserialized: FinalData = serde_json::from_str(&args.data).unwrap();

    //test_game(serde_json::from_str(&args.moves).unwrap());

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

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    // moves is the serde output
    stdin.write(&args.data);
    // Setup the prover client.
    let client = ProverClient::new();
    // Setup the program for proving.
    let (pk, vk) = client.setup(SOKOBAN_ELF);

    // Generate the proof, early fail if its wrong

    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("failed to generate proof");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");

    println!("Generating Proof ");

    println!("Proof generated successfully. Verifying proof...");
    println!("Proof verified successfully.");

    println!("Payment successful. Submitting proof...");

    // Serialize proof into bincode (format used by sp1)
    let proof = bincode::serialize(&proof).expect("Failed to serialize proof");

    println!("Payment successful. Submitting proof...");

    let raw_map = hex::decode(deserialized.map.clone()).expect("Decoding hex map failed");

    let pub_input_struct = PubInput {
        length: deserialized.length,
        rows: deserialized.rows,
        cols: deserialized.cols,
        player_col: deserialized.player_col,
        player_row: deserialized.player_row,
        map: raw_map,
    };

    // Get bytes ready for contract submission
    //let encoded: Bytes = pub_input_struct.encode_to_bytes();
    let encoded_vec: Vec<u8> = pub_input_struct.encode_to_vec();

    let verification_data = VerificationData {
        proving_system: ProvingSystemId::SP1,
        proof,
        proof_generator_addr: wallet.address(),
        vm_program_code: Some(SOKOBAN_ELF.to_vec()),
        verification_key: None,
        pub_input: Some(encoded_vec),
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
        &pub_input_struct.encode_to_vec(),
    )
    .await
    .expect("Claiming of NFT failed ...");
}

async fn claim_nft_with_verified_proof(
    aligned_verification_data: &AlignedVerificationData,
    signer: SignerMiddleware<Provider<Http>, LocalWallet>,
    verifier_contract_addr: &Address,
    pub_input: &Vec<u8>,
) -> anyhow::Result<()> {
    let verifier_contract = VerifierContract::new(*verifier_contract_addr, signer.into());

    let index_in_batch = U256::from(aligned_verification_data.index_in_batch);
    let merkle_path = Bytes::from(
        aligned_verification_data
            .batch_inclusion_proof
            .merkle_path
            .as_slice()
            .into_iter() // Convert the slice reference into an iterator
            .flatten() // Now flatten the iterator
            .copied() // Copy the elements because we are working with references
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

/*
fn test_game(deserialized: FinalData) {
    let input = deserialized.path;
    let total_moves = deserialized.length;

    let raw_map = hex::decode(deserialized.map.clone()).expect("Decoding hex map failed");

    // totalMoves number string to usize

    let input = input.trim();

    let moves_bytes = string_to_bytes(input);
    let moves = decode_moves(moves_bytes, total_moves);
    //println!("Decoded {} moves", moves.len());
    //println!("Moves sequence: {:?}", moves);
    let l = Level::new(deserialized.map, deserialized.rows, deserialized.cols);
    let mut game = Game::new(l.map, deserialized.player_row, deserialized.player_col);
    game.play(moves);
    println!("Game completed successfully");
}
    */