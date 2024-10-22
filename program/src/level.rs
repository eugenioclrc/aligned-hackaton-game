
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
}

pub fn base_level() -> Vec<Vec<Tile>>{
    let level = vec![
            vec![Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
            vec![Tile::Wall,  Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Empty, Tile::Box,   Tile::Empty, Tile::Box,   Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Empty, Tile::Wall,  Tile::Empty, Tile::Wall,  Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Target,Tile::Target,Tile::Player,Tile::Empty, Tile::Empty, Tile::Wall],
            vec![Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
            vec![Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
            vec![Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
];
            
        level
}

pub fn bytes32_to_bytes(bytes32: &str) -> [u8; 32] {
    // Remover el prefijo "0x" si existe
    let mut bytes = [0u8; 32];
    for (i, chunk) in bytes32
        .trim_start_matches("0x")
        .as_bytes()
        .chunks(2)
        .enumerate()
    {
        if i >= 32 {
            break;
        }
        let hex_str = std::str::from_utf8(chunk).unwrap_or("00");
        bytes[i] = u8::from_str_radix(hex_str, 16).unwrap_or(0);
    }
    bytes
}

pub fn uint256_to_level(level_bytes32: String) -> [[Tile; 8]; 8] {
    let level_bytes = bytes32_to_bytes(&level_bytes32);

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