#[derive(Clone, Copy, Debug)]
pub enum Direction {
    W = 0b00, // arriba    = 00
    A = 0b10, // izquierda = 10
    S = 0b01, // abajo     = 01
    D = 0b11, // derecha   = 11
}

impl Direction {
    pub fn from_u8(value: u8) -> Option<Direction> {
        match value & 0b11 {
            0b00 => Some(Direction::W),
            0b10 => Some(Direction::A),
            0b01 => Some(Direction::S),
            0b11 => Some(Direction::D),
            _ => None,
        }
    }
}

pub fn decode_moves(moves_bytes: Vec<u8>, total_moves: u32) -> Vec<Direction> {
    // Convertir string hex a bytes

    let mut moves = Vec::new();

    // Decodificar cada par de bits en una dirección
    for byte in moves_bytes {
        for shift in (0..8).step_by(2).rev() {
            let dir_bits = (byte >> shift) & 0b11;
            if let Some(dir) = Direction::from_u8(dir_bits) {
                moves.push(dir);
            }
            if (moves.len() as u32 == total_moves) {
                break;
            }
        }
    }

    moves
}
