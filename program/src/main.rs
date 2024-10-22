//! A simple program to be proven inside the zkVM.
//! Consists in a 5 question multiple choice quiz
//! with 3 possible answers each.

#![no_main]

sp1_zkvm::entrypoint!(main);
use serde::{Deserialize, Serialize};

mod directions;
use directions::decode_moves;

mod game;
use game::Game;

mod level;
use level::base_level;

#[derive(Serialize, Deserialize)]
struct FinalData {
    path: String,
    pub length: u8,
    // set the user map as public input
    pub map: String,
}


fn bytes32_to_bytes(bytes32: &str) -> [u8; 32] {
    // Convertir string hex a bytes
    let mut bytes = [0u8; 32];
    for (i, chunk) in bytes32.trim_start_matches("0x").as_bytes().chunks(2).enumerate() {
        if i >= 32 { break; }
        let hex_str = std::str::from_utf8(chunk).unwrap_or("00");
        bytes[i] = u8::from_str_radix(hex_str, 16).unwrap_or(0);
    }
    bytes
}


fn main() {
    
    let zkinput = sp1_zkvm::io::read::<String>();

    let deserialized: FinalData = serde_json::from_str(&zkinput).unwrap();

    let input = deserialized.path;
    let total_moves = deserialized.length;

    // commit the score
    sp1_zkvm::io::commit::<u8>(&total_moves);
    sp1_zkvm::io::commit::<String>(&deserialized.map); // should i also commit the map?

    // totalMoves number string to usize

    let input = input.trim();

    let moves_bytes = bytes32_to_bytes(input);
    let moves = decode_moves(moves_bytes);
    //println!("Decoded {} moves", moves.len());
    //println!("Moves sequence: {:?}", moves);

    let mut game = Game::new(base_level());
    
    //println!("\nInitial state:");
    //game.print();
    //println!("\nPress Enter to start simulation...");
    //io::stdin().read_line(&mut String::new()).unwrap();

    for (_i, &direction) in moves.iter().enumerate() {
        if !game.move_player(direction) || game.is_won() {
            break;
        }
    }

    if !game.is_won() {
        panic!("Solution did not solve the puzzle!");
        //println!("\nSolution did not solve the puzzle!");
    }
    if game.moves != total_moves {
        panic!("Solution has different moves than expected!");
    }
}