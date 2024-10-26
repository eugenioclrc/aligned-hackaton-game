//! A simple program to be proven inside the zkVM.
//! Consists in a classic puzzle, the sokoban game.

#![no_main]

sp1_zkvm::entrypoint!(main);
use serde::{Deserialize, Serialize};

use sokoban::{decode_moves, string_to_bytes, Game, Level};

#[derive(Serialize, Deserialize)]
struct FinalData {
    // this is private data
    path: String,
    length: u32,
    // this is the map info
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
fn main() {
    let zkinput = sp1_zkvm::io::read::<String>();

    let deserialized: FinalData = serde_json::from_str(&zkinput).unwrap();

    let input = deserialized.path;
    let total_moves = deserialized.length;

    let raw_map = hex::decode(deserialized.map.clone()).expect("Decoding hex map failed");

    // commit the score
    sp1_zkvm::io::commit::<PubInput>(&PubInput {
        length: total_moves as u128,
        rows: deserialized.rows,
        cols: deserialized.cols,
        player_col: deserialized.player_col,
        player_row: deserialized.player_row,
        map: raw_map,
    });

    // totalMoves number string to usize

    let input = input.trim();

    let moves_bytes = string_to_bytes(input);
    let moves = decode_moves(moves_bytes, total_moves);
    //println!("Decoded {} moves", moves.len());
    //println!("Moves sequence: {:?}", moves);
    let l = Level::new(deserialized.map, deserialized.rows, deserialized.cols);
    let mut game = Game::new(l.map, deserialized.player_row, deserialized.player_col);

    //println!("\nInitial state:");
    //game.print();
    //println!("\nPress Enter to start simulation...");
    //io::stdin().read_line(&mut String::new()).unwrap();
    game.play(moves);

}
