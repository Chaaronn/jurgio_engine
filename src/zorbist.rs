use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use crate::pieces::{PieceColour, PieceKind};

/// Represents Zobrist keys for hashing the board state.
pub struct ZobristHashing {
    pub piece_keys: [[[u64; 64]; 6]; 2], // [colour][piece kind][square]
    pub side_to_move_key: u64,
    pub castling_keys: [u64; 16],
    pub en_passant_keys: [u64; 8],
}

impl ZobristHashing {
    /// Initialize Zobrist keys with random values.
    pub fn new() -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(42); // Seed for reproducibility

        // Generate keys for pieces on squares
        let mut piece_keys = [[[0u64; 64]; 6]; 2];
        for colour in 0..2 {
            for piece in 0..6 {
                for square in 0..64 {
                    piece_keys[colour][piece][square] = rng.gen();
                }
            }
        }

        // Generate side to move key
        let side_to_move_key = rng.gen();

        // Generate castling keys (16 combinations: 4 castling rights per player)
        let mut castling_keys = [0u64; 16];
        for i in 0..16 {
            castling_keys[i] = rng.gen();
        }

        // Generate en passant keys (1 key for each file)
        let mut en_passant_keys = [0u64; 8];
        for file in 0..8 {
            en_passant_keys[file] = rng.gen();
        }

        Self {
            piece_keys,
            side_to_move_key,
            castling_keys,
            en_passant_keys,
        }
    }

    /// Compute the Zobrist hash for the given board state.
    pub fn compute_hash(&self, board: &crate::board::BoardState) -> u64 {
        let mut hash = 0u64;
    
        // Include piece positions in hash
        for square in 0..64 {
            if let Some(piece) = board.piece_at(square) {
                let colour_index = match piece.colour {
                    PieceColour::White => 0,
                    PieceColour::Black => 1,
                };
                let piece_index = match piece.kind {
                    PieceKind::Pawn => 0,
                    PieceKind::Knight => 1,
                    PieceKind::Bishop => 2,
                    PieceKind::Rook => 3,
                    PieceKind::Queen => 4,
                    PieceKind::King => 5,
                };
                hash ^= self.piece_keys[colour_index][piece_index][square];
            }
        }
    
        // Include side to move in hash
        if board.to_move == PieceColour::Black {
            hash ^= self.side_to_move_key;
        }
    
        // Include castling rights in hash
        let castling_index = board.get_castling_rights_index();
        hash ^= self.castling_keys[castling_index];
    
        // Include en passant square in hash (if any)
        if let Some(ep_file) = board.en_passant_square {
            let file = ep_file % 8;
            hash ^= self.en_passant_keys[file];
        }
    
        hash
    }
    
}

impl crate::board::BoardState {
    /// Get the castling rights index.
    pub fn get_castling_rights_index(&self) -> usize {
        let mut index = 0;
        if self.can_castle_kingside(PieceColour::White) {
            index |= 1 << 0;
        }
        if self.can_castle_queenside(PieceColour::White) {
            index |= 1 << 1;
        }
        if self.can_castle_kingside(PieceColour::Black) {
            index |= 1 << 2;
        }
        if self.can_castle_queenside(PieceColour::Black) {
            index |= 1 << 3;
        }
        index
    }

    /// Mock implementation: Get the en passant file.
    pub fn get_en_passant_file(&self) -> Option<usize> {
        // TODO: Implement proper en passant tracking
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BoardState;

    #[test]
    fn test_zobrist_hashing_initial_board() {
        let zobrist = ZobristHashing::new();
        let board = BoardState::new();

        let hash = zobrist.compute_hash(&board);
        println!("Initial board hash: {}", hash);

        // Assert hash is non-zero
        assert!(hash != 0);
    }
}
