//! Test for the game lib that sp1 zkevm will use.

use serde::{Deserialize, Serialize};

mod directions;
use directions::decode_moves;

mod game;
use game::Game;

mod level;
use level::{base_level, string_to_bytes};

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
    
    //let zkinput = sp1_zkvm::io::read::<String>();
    let zkinput = "{\"rows\":6,\"cols\":7,\"map\":\"aaaa002844a222bc0aaaa0\",\"player_row\":2,\"player_col\":1,\"path\":\"3FD89894F4F5A\",\"length\":26}";

    let deserialized: FinalData = serde_json::from_str(&zkinput).unwrap();

    let input = deserialized.path;
    let total_moves = deserialized.length;

    // commit the score
    
    // sp1_zkvm::io::commit::<u8>(&total_moves);
    println!("commiting the score {:?}", total_moves);
    //sp1_zkvm::io::commit::<String>(&deserialized.map); // should i also commit the map?
    println!("commiting the map {:?}", deserialized.map);
    // totalMoves number string to usize

    let input = input.trim();

    let moves_bytes = string_to_bytes(input);

    println!("__Moves bytes: {:?}", input.len());
    println!("__Moves bytes: {:?}", moves_bytes.len());
    let moves = decode_moves(moves_bytes, total_moves);
    
    //println!("Decoded {} moves", moves.len());
    //println!("Moves sequence: {:?}", moves);

    let mut game = Game::new(base_level(), deserialized.player_row, deserialized.player_col);
    
    //println!("\nInitial state:");
    //game.print();
    //println!("\nPress Enter to start simulation...");
    //io::stdin().read_line(&mut String::new()).unwrap();

    // print moves
    println!("Moves: {:?}", moves);
    println!("Total moves: {:?}", moves.len());

    for (_i, &direction) in moves.iter().enumerate() {
        if(_i > total_moves as usize) {
            break;
        }
        game.print_level();
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
    println!("Solution is correct!");
    println!("Total moves: {:?}", game.moves);
}