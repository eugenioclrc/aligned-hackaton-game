use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use hex;

#[derive(Serialize, Deserialize)]
struct FinalData {
    path: String,
    length: u8,
}



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

#[derive(Clone, Copy)]
enum Direction {
    W = 0b00,  // arriba    = 00
    A = 0b10,  // izquierda = 10
    S = 0b01,  // abajo     = 01
    D = 0b11   // derecha   = 11
}

impl Direction {
    fn to_u8(self) -> u8 {
        self as u8
    }
}

struct Game {
    map: Vec<Vec<Tile>>,
    player_pos: (usize, usize),
    moves: usize,
}

// Nueva estructura para manejar la codificación de direcciones
struct DirectionEncoder {
    data: [u8; 32],
    current_byte: usize,
    current_bit: usize,
}

impl DirectionEncoder {
    fn new() -> Self {
        Self {
            data: [0u8; 32],
            current_byte: 0,
            current_bit: 0,
        }
    }

    // Add a method to get the current data and length
    fn get_data(&self) -> (([u8; 32]), u8) {
        let length = if self.current_bit == 0 {
            (self.current_byte * 4) as u8
        } else {
            (self.current_byte * 4 + (self.current_bit / 2)) as u8
        };
        (self.data, length)
    }
    

    fn add_direction(&mut self, direction: Direction) -> bool {
        if self.current_byte >= 32 {
            return false; // No hay más espacio
        }

        // Obtener los 2 bits de la dirección
        let dir_bits = direction.to_u8() & 0b11;
        
        // Calcular la posición en el byte actual
        let shift = 6 - self.current_bit;
        
        // Añadir los bits al byte actual
        self.data[self.current_byte] |= dir_bits << shift;
        
        // Actualizar las posiciones
        self.current_bit += 2;
        if self.current_bit >= 8 {
            self.current_bit = 0;
            self.current_byte += 1;
        }
        
        true
    }

    fn print_encoded(&self) {
        println!("\nEncoded directions (bytes32):");
        print!("0x");
        for byte in &self.data {
            print!("{:02x}", byte);
        }
        println!();

        // Imprimir representación binaria de los bytes usados
        println!("\nBinary representation of used bytes:");
        let used_bytes = if self.current_bit == 0 {
            self.current_byte
        } else {
            self.current_byte + 1
        };

        for i in 0..used_bytes {
            print!("Byte {}: ", i);
            for bit in (0..8).rev() {
                print!("{}", (self.data[i] >> bit) & 1);
                if bit % 2 == 0 {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

impl Game {
    // ... [resto del código de Game se mantiene igual] ...
    fn new() -> Self {
        // Define a more challenging level
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
                    
                    // Move box
                    self.map[box_new_x][box_new_y] = if self.map[box_new_x][box_new_y] == Tile::Target { Tile::BoxOnTarget } else { Tile::Box };
                    
                    // Move player
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

fn main() {
    let mut game = Game::new();
    let mut encoder = DirectionEncoder::new();
    
    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen

        let (path, length) = encoder.get_data();
        let data = FinalData {
            path: format!("0x{}", hex::encode(path)),
            length,
        };

        let serialized = serde_json::to_string(&data).unwrap();
        println!("{}", serialized);

        game.print();
        
        if game.is_won() {
            println!("Congratulations! You won in {} moves!", game.moves);
            encoder.print_encoded();
            break;
        }
        
        print!("Enter move (w/a/s/d/q to quit): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let direction = match input.trim() {
            "w" => Some(Direction::W),
            "s" => Some(Direction::S),
            "a" => Some(Direction::A),
            "d" => Some(Direction::D),
            "q" => break,
            _ => None,
        };

        if let Some(dir) = direction {
            if !encoder.add_direction(dir) {
                println!("Warning: Maximum moves reached, cannot encode more directions");
            }
            game.move_player(dir);
        }
    }
}