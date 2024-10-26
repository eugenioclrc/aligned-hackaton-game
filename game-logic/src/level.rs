#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Empty,
    Wall,
    Player,
    Box,
    Target,
    BoxOnTarget,
    PlayerOnTarget,
}

impl Tile {
    pub fn from_u8(value: u8) -> Option<Tile> {
        match value & 0b11 {
            0b00 => Some(Tile::Empty),
            0b10 => Some(Tile::Wall),
            0b01 => Some(Tile::Box),
            0b11 => Some(Tile::Target),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Level {
    pub rows: usize,
    pub cols: usize,
    pub map: Vec<Vec<Tile>>,
    pub hex_map: String,
}

impl Level {
    pub fn new(map: String, rows: u32, cols: u32) -> Self {
        let hex_map = map.clone();

        Level {
            rows: rows as usize,
            cols: cols as usize,
            hex_map: hex_map,
            map: bytes_to_level(map, rows, cols),
        }
    }
}

pub fn bytes_to_level(level_bytes: String, rows: u32, cols: u32) -> Vec<Vec<Tile>> {
    let level_bytes = string_to_bytes(&level_bytes);
    // Check if the byte length is enough to cover all the tiles
    let required_length = ((rows * cols + 3) / 4) as usize;
    if level_bytes.len() < required_length {
        panic!("Invalid level bytes length");
    }

    let mut level = Vec::new();

    for row in 0..rows as usize {
        let mut current_row: Vec<Tile> = Vec::new();
        for col in 0..cols as usize {
            let tile_index = row * cols as usize + col;
            let byte_index = tile_index / 4;
            let bit_offset = (tile_index % 4) * 2;
            // Extract the correct 2 bits for the tile
            let tile_bits = (level_bytes[byte_index] >> (6 - bit_offset)) & 0b11;

            if let Some(tile) = Tile::from_u8(tile_bits) {
                current_row.push(tile);
            } else {
                panic!("Invalid tile bits encountered");
            }
        }
        level.push(current_row);
    }

    level
}

pub fn string_to_bytes(hexa_string: &str) -> Vec<u8> {
    let mut hex = hexa_string.trim_start_matches("0x").to_string();

    if hex.len() % 2 != 0 {
        hex = format!("{}0", hex); // Append '0' to make it even length
    }

    let mut bytes = Vec::<u8>::with_capacity(hex.len() / 2);
    for chunk in hex.as_bytes().chunks(2) {
        let hex_str = std::str::from_utf8(chunk).unwrap_or("00");
        bytes.push(u8::from_str_radix(hex_str, 16).unwrap_or(0));
    }
    bytes
}
