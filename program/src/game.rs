
use crate::directions::Direction;
use crate::level::Tile;

pub struct Game {
    pub map: Vec<Vec<Tile>>,
    pub player_pos: (usize, usize),
    pub moves: u8,
}

impl Game {
    pub fn new(level: Vec<Vec<Tile>>) -> Self {
        
        Game {
            player_pos: (4, 3),
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

    pub fn move_player(&mut self, direction: Direction) -> bool {
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