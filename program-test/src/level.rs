
#[derive(Clone, Copy, PartialEq)]
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

    fn to_bits(&self) -> u8 {
        match self {
            Tile::Empty => 0b00,
            Tile::Wall => 0b10,
            Tile::Box => 0b01,
            Tile::Target => 0b11,
            _ => 0b00, // Default to empty
        }
    }
}

#[derive(Debug)]
struct Level {
    row: usize,
    cols: usize,
    map: String,  // The map is a string representing hex data
}

// Function to convert hex character to its numeric value
fn hex_char_to_u8(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - '0' as u8,
        'a'..='f' => 10 + (c as u8 - 'a' as u8),
        'A'..='F' => 10 + (c as u8 - 'A' as u8),
        _ => panic!("Invalid hex character"),
    }
}

impl Level {

    // Converts hex string into Vec<Vec<Option<Tile>>>
    fn to_tile_vec(&self) -> Vec<Vec<Option<Tile>>> {
        // Remove "0x" prefix if present
        let clean_hex = self.map.trim_start_matches("0x");

        // Assert that the hex string is long enough to store the map
        let required_hex_length = (self.row * self.cols) / 2;
        assert!(
            clean_hex.len() >= required_hex_length,
            "Hex string is too short to represent the entire map!"
        );

        let mut tiles = Vec::new();
        let mut bytes = Vec::new();

        // Convert the hexadecimal string into bytes manually
        let chars: Vec<char> = clean_hex.chars().collect();
        for i in (0..chars.len()).step_by(2) {
            let high = hex_char_to_u8(chars[i]) << 4;
            let low = hex_char_to_u8(chars[i + 1]);
            bytes.push(high | low);  // Combine the high and low nibbles to form a byte
        }

        // Iterate over the rows
        for row in 0..self.row {
            let mut row_tiles = Vec::new();

            // Iterate over the columns
            for col in 0..self.cols {
                let tile_index = row * self.cols + col;

                // Each tile is 2 bits, so divide by 4 (since 1 byte can hold 4 tiles)
                let byte_index = tile_index / 4;
                let bit_offset = (tile_index % 4) * 2;

                // Get the byte and extract the 2 bits that represent the tile
                let byte = bytes[byte_index];
                let bits = (byte >> (6 - bit_offset)) & 0b11;  // Shift to get the correct 2 bits

                // Convert the bits to a Tile
                let tile = Tile::from_u8(bits);
                row_tiles.push(tile);
            }

            tiles.push(row_tiles);
        }

        tiles
    }

    // Function to convert Vec<Vec<Tile>> into a Map with hex string
    fn from_tile_vec(tiles: Vec<Vec<Tile>>) -> Self {
        let rows = tiles.len();
        let cols = tiles[0].len();
        let total_tiles = rows * cols;

        let mut byte_vec = vec![0u8; (total_tiles + 3) / 4];  // 2 bits per tile, 4 tiles per byte

        // Iterate over the tiles and pack them into the byte array
        for row in 0..rows {
            for col in 0..cols {
                let tile_index = row * cols + col;
                let byte_index = tile_index / 4;
                let bit_offset = (tile_index % 4) * 2;

                let bits = tiles[row][col].to_bits();
                byte_vec[byte_index] |= bits << (6 - bit_offset);
            }
        }

        // Convert the byte array into a hexadecimal string
        let hex_map: String = byte_vec.iter().map(|byte| format!("{:02x}", byte)).collect();

        Level {
            row: rows,
            cols: cols,
            map: hex_map,
        }
    }
}

pub fn base_level() -> Vec<Vec<Tile>>{
    let level = vec![
            vec![Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
            vec![Tile::Wall,  Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Empty, Tile::Box,   Tile::Empty, Tile::Box,   Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Empty, Tile::Wall,  Tile::Empty, Tile::Wall,  Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Target,Tile::Target,Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
          
];
            
        level
}


pub fn uint256_to_level(level_bytes32: String) -> [[Tile; 8]; 8] {
    let level_bytes = string_to_bytes(&level_bytes32);

    let mut level = [[Tile::Empty; 8]; 8];

    for row in 0..8 {
        for col in 0..8 {
            let shift_amount = (row * 8 + col) * 2;
            let byte_index = shift_amount / 8;
            let bit_index = shift_amount % 8;
            let tile_bits = ((level_bytes[byte_index] >> bit_index) & 0b11) as u8;
            if let Some(tile) = Tile::from_u8(tile_bits) {
                level[row][col] = tile;
            }
        }
    }

    level
}
pub fn string_to_bytes(hexa_string: &str) -> Vec<u8> {
    let mut hex = hexa_string.trim_start_matches("0x").to_string();

    // Si la longitud es impar, agregar un "0" al principio
    if hex.len() % 2 != 0 {
        hex = format!("{}0", hex);  // Prepending '0' to make it even length
    }

    let mut bytes = Vec::<u8>::with_capacity(hex.len() / 2);
    for chunk in hex.as_bytes().chunks(2) {
        let hex_str = std::str::from_utf8(chunk).unwrap_or("00");
        bytes.push(u8::from_str_radix(hex_str, 16).unwrap_or(0));
    }
    bytes
}
