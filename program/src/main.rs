#![no_main]
sp1_zkvm::entrypoint!(main);

// use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Player,
    Box,
    Target,
    BoxOnTarget,
    PlayerOnTarget,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    W = 0b00,  // arriba    = 00
    A = 0b10,  // izquierda = 10
    S = 0b01,  // abajo     = 01
    D = 0b11   // derecha   = 11
}

impl Direction {
    fn from_u8(value: u8) -> Option<Direction> {
        match value & 0b11 {
            0b00 => Some(Direction::W),
            0b10 => Some(Direction::A),
            0b01 => Some(Direction::S),
            0b11 => Some(Direction::D),
            _ => None
        }
    }
}

struct Game {
    map: Vec<Vec<Tile>>,
    player_pos: (usize, usize),
    moves: usize,
}

impl Game {
    fn new() -> Self {
        let level = vec![
            vec![Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
            vec![Tile::Wall,  Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Empty, Tile::Box,   Tile::Empty, Tile::Box,   Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Empty, Tile::Wall,  Tile::Empty, Tile::Wall,  Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Target,Tile::Target,Tile::Player,Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
        ];

        Game {
            player_pos: (4, 3),
            map: level,
            moves: 0,
        }
    }

    fn print(&self) {
        for row in &self.map {
            for tile in row {
                let symbol = match tile {
                    Tile::Empty => ' ',
                    Tile::Wall => '#',
                    Tile::Player => '@',
                    Tile::Box => '$',
                    Tile::Target => '.',
                    Tile::BoxOnTarget => '*',
                    Tile::PlayerOnTarget => '+',
                };
                print!("{}", symbol);
            }
            println!();
        }
        println!("Moves: {}", self.moves);
    }

    fn is_won(&self) -> bool {
        for row in &self.map {
            for tile in row {
                if *tile == Tile::Target || *tile == Tile::PlayerOnTarget {
                    return false;
                }
            }
        }
        true
    }

    fn move_player(&mut self, direction: Direction) -> bool {
        let dx = match direction {
            Direction::W => -1,
            Direction::S => 1,
            _ => 0,
        };
        let dy = match direction {
            Direction::A => -1,
            Direction::D => 1,
            _ => 0,
        };
        
        let new_x = self.player_pos.0 as i32 + dx;
        let new_y = self.player_pos.1 as i32 + dy;
        
        if new_x < 0 || new_y < 0 {
            return false;
        }
        
        let new_x = new_x as usize;
        let new_y = new_y as usize;

        match self.map[new_x][new_y] {
            Tile::Empty | Tile::Target => {
                let was_on_target = self.map[self.player_pos.0][self.player_pos.1] == Tile::PlayerOnTarget;
                self.map[self.player_pos.0][self.player_pos.1] = if was_on_target { Tile::Target } else { Tile::Empty };
                self.map[new_x][new_y] = if self.map[new_x][new_y] == Tile::Target { Tile::PlayerOnTarget } else { Tile::Player };
                self.player_pos = (new_x, new_y);
                self.moves += 1;
                true
            }
            Tile::Box | Tile::BoxOnTarget => {
                let box_new_x = new_x as i32 + dx;
                let box_new_y = new_y as i32 + dy;
                
                if box_new_x < 0 || box_new_y < 0 {
                    return false;
                }
                
                let box_new_x = box_new_x as usize;
                let box_new_y = box_new_y as usize;

                if self.map[box_new_x][box_new_y] == Tile::Empty || self.map[box_new_x][box_new_y] == Tile::Target {
                    let was_on_target = self.map[self.player_pos.0][self.player_pos.1] == Tile::PlayerOnTarget;
                    let box_was_on_target = self.map[new_x][new_y] == Tile::BoxOnTarget;
                    
                    self.map[box_new_x][box_new_y] = if self.map[box_new_x][box_new_y] == Tile::Target { Tile::BoxOnTarget } else { Tile::Box };
                    self.map[self.player_pos.0][self.player_pos.1] = if was_on_target { Tile::Target } else { Tile::Empty };
                    self.map[new_x][new_y] = if box_was_on_target { Tile::PlayerOnTarget } else { Tile::Player };
                    
                    self.player_pos = (new_x, new_y);
                    self.moves += 1;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

fn decode_moves(bytes32: &str) -> Vec<Direction> {
    let mut moves = Vec::new();
    
    // Remover el prefijo "0x" si existe
    let bytes32 = bytes32.trim_start_matches("0x");
    
    // Convertir string hex a bytes
    let mut bytes = [0u8; 32];
    for (i, chunk) in bytes32.as_bytes().chunks(2).enumerate() {
        if i >= 32 { break; }
        let hex_str = std::str::from_utf8(chunk).unwrap_or("00");
        bytes[i] = u8::from_str_radix(hex_str, 16).unwrap_or(0);
    }

    // Decodificar cada par de bits en una dirección
    for byte in bytes {
        for shift in (0..8).step_by(2).rev() {
            let dir_bits = (byte >> shift) & 0b11;
            if let Some(dir) = Direction::from_u8(dir_bits) {
                moves.push(dir);
            }
        }
    }

    moves
}

fn main() {
    println!("Enter the bytes32 solution (with or without 0x prefix):");
    let input = sp1_zkvm::io::read::<String>();

    let input = input.trim();

    let moves = decode_moves(input);
    println!("Decoded {} moves", moves.len());
    println!("Moves sequence: {:?}", moves);

    let mut game = Game::new();
    //let mut step = 0;
    
    println!("\nInitial state:");
    game.print();
    println!("\nPress Enter to start simulation...");
    //io::stdin().read_line(&mut String::new()).unwrap();

    for (i, &direction) in moves.iter().enumerate() {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen
        println!("Executing move {} of {}: {:?}", i + 1, moves.len(), direction);
        
        if game.move_player(direction) {
            game.print();
        } else {
            println!("Invalid move!");
        }

        if game.is_won() {
            println!("\nSolution found in {} moves!", game.moves);
            break;
        }

        // Pequeña pausa entre movimientos
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    if !game.is_won() {
        println!("\nSolution did not solve the puzzle!");
    }
}