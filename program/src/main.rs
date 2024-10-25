//! A simple program to be proven inside the zkVM.
//! Consists in a classic puzzle, the sokoban game.

#![no_main]

sp1_zkvm::entrypoint!(main);
use serde::{Deserialize, Serialize};

mod directions;
use directions::decode_moves;

mod game;
use game::Game;

mod level;
use level::{string_to_bytes, Level};

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
    map: String
}
fn main() {
    
    let zkinput = sp1_zkvm::io::read::<String>();

    let deserialized: FinalData = serde_json::from_str(&zkinput).unwrap();

    let input = deserialized.path;
    let total_moves = deserialized.length;

    // commit the score
    sp1_zkvm::io::commit::<u32>(&total_moves);
    sp1_zkvm::io::commit::<String>(&deserialized.map); // should i also commit the map?

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