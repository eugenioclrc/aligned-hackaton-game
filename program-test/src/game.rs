use crate::directions::Direction;
use crate::level::Tile;

pub struct Game {
    pub map: Vec<Vec<Tile>>,
    pub player_pos: (u32, u32), // (row, col)
    pub moves: u32,
}

impl Game {
    pub fn new(mut level: Vec<Vec<Tile>>, row: u32, col: u32) -> Self {
        if level[row as usize][col as usize] != Tile::Empty {
            panic!("Player position is invalid");
        }

        level[row as usize][col as usize] = Tile::Player;
        Game {
            player_pos: (row, col),
            map: level,
            moves: 0,
        }
    }

    pub fn is_won(&self) -> bool {
        for row in &self.map {
            for tile in row {
                if *tile == Tile::Target || *tile == Tile::PlayerOnTarget {
                    return false;
                }
            }
        }
        true
    }

    pub fn print_level(&self) {
        for row in &self.map {
            for tile in row {
                match tile {
                    Tile::Player | Tile::PlayerOnTarget => print!("P"),
                    Tile::Box | Tile::BoxOnTarget => print!("B"),
                    Tile::Wall => print!("#"),
                    Tile::Target => print!("T"),
                    _ => print!(" "),
                }
            }
            println!();
        }
    }

    pub fn move_player(&mut self, direction: Direction) -> bool {
        let dy = match direction {
            Direction::W => -1,
            Direction::S => 1,
            _ => 0,
        };
        let dx = match direction {
            Direction::A => -1,
            Direction::D => 1,
            _ => 0,
        };
        
        let new_row = self.player_pos.0 as i32 + dy;
        let new_col = self.player_pos.1 as i32 + dx;
        
        if new_row < 0 || new_col < 0 {
            return false;
        }
        
        let prev_row = self.player_pos.0 as usize;
        let prev_col = self.player_pos.1 as usize;

        let new_row = new_row as usize;
        let new_col = new_col as usize;

        println!("New position: ({}, {})", new_row, new_col);

        match self.map[new_row][new_col] {
            Tile::Empty | Tile::Target => {
                let was_on_target = self.map[prev_row][prev_col] == Tile::PlayerOnTarget;
                self.map[prev_row][prev_col] = if was_on_target { Tile::Target } else { Tile::Empty };
                self.map[new_row][new_col] = if self.map[new_row][new_col] == Tile::Target { Tile::PlayerOnTarget } else { Tile::Player };
                self.player_pos = (new_row as u32, new_col as u32);
                self.moves += 1;

                true
            }
            Tile::Box | Tile::BoxOnTarget => {
                let box_new_row = new_row as i32 + dy;
                let box_new_col = new_col as i32 + dx;
                
                if box_new_row < 0 || box_new_col < 0 {
                    return false;
                }
                
                let box_new_row = box_new_row as usize;
                let box_new_col = box_new_col as usize;

                if self.map[box_new_row][box_new_col] == Tile::Empty || self.map[box_new_row][box_new_col] == Tile::Target {
                    let was_on_target = self.map[prev_row][prev_col] == Tile::PlayerOnTarget;
                    let box_was_on_target = self.map[new_row][new_col] == Tile::BoxOnTarget;
                    
                    self.map[box_new_row][box_new_col] = if self.map[box_new_row][box_new_col] == Tile::Target { Tile::BoxOnTarget } else { Tile::Box };
                    self.map[prev_row][prev_col] = if was_on_target { Tile::Target } else { Tile::Empty };
                    self.map[new_row][new_col] = if box_was_on_target { Tile::PlayerOnTarget } else { Tile::Player };
                    
                    self.player_pos = (new_row as u32, new_col as u32);
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
