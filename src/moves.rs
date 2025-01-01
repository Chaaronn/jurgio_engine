use crate::board::{BoardState, BitBoard};
use crate::pieces::{PieceColour, PieceKind};
use tracing;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ChessMove {
    pub from: usize, // Single index (0-63)
    pub to: usize,   // Single index (0-63)
    pub promotion: Option<PieceKind>,
}

impl BoardState {
    /// Generates all valid moves for the current player.
    pub fn generate_moves(&mut self) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        match self.to_move {
            PieceColour::White => {
                let white_pieces = self.all_white; // Copy the bitboard
                self.generate_colour_moves(&white_pieces, &mut moves);
            }
            PieceColour::Black => {
                let black_pieces = self.all_black; // Copy the bitboard
                self.generate_colour_moves(&black_pieces, &mut moves);
            }
        }

        moves
    }

    /// Generate moves for a specific color.
    fn generate_colour_moves(&mut self, pieces: &BitBoard, moves: &mut Vec<ChessMove>) {
        tracing::debug!("All white bitboard: {:064b}", self.all_white.0);
        for square in pieces.iter() {
            tracing::debug!("Iterating square: {}", square);
            if let Some(piece) = self.piece_at(square) {
                tracing::debug!("Processing piece: {:?} at square {}", piece, square);
                if piece.colour == self.to_move {
                    match piece.kind {
                        PieceKind::Pawn => self.generate_pawn_moves(square, piece.colour, moves),
                        PieceKind::Knight => self.generate_knight_moves(square, moves),
                        PieceKind::Bishop => self.generate_bishop_moves(square, moves),
                        PieceKind::Rook => self.generate_rook_moves(square, moves),
                        PieceKind::Queen => self.generate_queen_moves(square, moves),
                        PieceKind::King => self.generate_king_moves(square, moves),
                    }
                }
            }
        }
    }

    /// Generate pawn moves, including promotions and en passant.
    fn generate_pawn_moves(&mut self, square: usize, colour: PieceColour, moves: &mut Vec<ChessMove>) {
        let direction = if colour == PieceColour::White { 8 } else { -8 };
        let forward = square as isize + direction;

        // Single forward move
        if forward >= 0 && forward < 64 && !self.all_pieces.is_set(forward as usize) {
            moves.push(ChessMove {
                from: square,
                to: forward as usize,
                promotion: self.promotion_check(forward as usize, colour),
            });

            // Double forward move from starting rank
            if self.is_pawn_starting_rank(square, colour) {
                let double_forward = square as isize + 2 * direction;
                if double_forward >= 0 && double_forward < 64 && !self.all_pieces.is_set(double_forward as usize) {
                    tracing::debug!(
                        "Checking two-square move for pawn at {}: direction={} double_forward={}",
                        square,
                        direction,
                        double_forward
                    );

                    moves.push(ChessMove {
                        from: square,
                        to: double_forward as usize,
                        promotion: None,
                    });
            
                    // Set en passant square for the opponent only on a valid two-square move
                    self.en_passant_square = Some((square as isize + direction) as usize);
                    tracing::debug!("Set en_passant_square={:?}", self.en_passant_square);
                }
            }
        }

        // Captures
        let capture_offsets = if colour == PieceColour::White { [-9, -7] } else { [7, 9] };
        for &offset in &capture_offsets {
            let target = square as isize + offset;

            // Standard capture
            if target >= 0
                && target < 64
                && self.all_pieces.is_set(target as usize)
                && self.is_opponent_piece(target as usize, colour)
            {
                moves.push(ChessMove {
                    from: square,
                    to: target as usize,
                    promotion: self.promotion_check(target as usize, colour),
                });
            }

            // En passant capture
            if let Some(ep_square) = self.en_passant_square {
                if (square == ep_square - 9 || square == ep_square - 7 || // White pawn capture
                    square == ep_square + 9 || square == ep_square + 7) { // Black pawn capture
                    moves.push(ChessMove {
                        from: square,
                        to: ep_square,
                        promotion: None,
                    });
                    tracing::debug!("Generated en passant move from {} to {}", square, ep_square);
                } else {
                    tracing::debug!(
                        "Skipped en passant for square {}: no legal pawn to capture ep_square={}",
                        square,
                        ep_square
                    );
                }
            }

            
            
        }
    }

    /// Check if a square contains an opponent's piece.
    fn is_opponent_piece(&self, square: usize, colour: PieceColour) -> bool {
        match colour {
            PieceColour::White => self.all_black.is_set(square),
            PieceColour::Black => self.all_white.is_set(square),
        }
    }

    /// Determine if a pawn is on its starting rank.
    fn is_pawn_starting_rank(&self, square: usize, colour: PieceColour) -> bool {
        match colour {
            PieceColour::White => (8..16).contains(&square),
            PieceColour::Black => (48..56).contains(&square),
        }
    }

    /// Check if a pawn move results in promotion.
    fn promotion_check(&self, square: usize, colour: PieceColour) -> Option<PieceKind> {
        match colour {
            PieceColour::White if square < 8 => Some(PieceKind::Queen),
            PieceColour::Black if square >= 56 => Some(PieceKind::Queen),
            _ => None,
        }
    } 
    
    /// Generate knight moves.
    fn generate_knight_moves(&self, square: usize, moves: &mut Vec<ChessMove>) {
    
        let offsets = [17, 15, 10, 6, -17, -15, -10, -6];
        let rank = (square / 8) as isize; // Current rank (0 to 7)
        let file = (square % 8) as isize; // Current file (0 to 7)
    
        for &offset in &offsets {
            let target = square as isize + offset;
    
            // Check if target is on the board
            if target >= 0 && target < 64 {
                let target_rank = target / 8;
                let target_file = target % 8;
    
                // Validate file difference for wrapping prevention
                let file_diff = (target_file - file).abs();
                tracing::debug!(
                    target,
                    target_rank,
                    target_file,
                    file_diff,
                    "Calculating knight move"
                );
    
                // Ensure the move stays within valid ranks and files
                if (offset.abs() == 17 || offset.abs() == 15) && file_diff == 1
                    || (offset.abs() == 10 || offset.abs() == 6) && file_diff == 2
                {
                    tracing::debug!(from = square, to = target, "Adding knight move");
                    moves.push(ChessMove {
                        from: square,
                        to: target as usize,
                        promotion: None,
                    });
                }
            }
        }
    }

    /// Generate bishop moves.
    fn generate_bishop_moves(&self, square: usize, moves: &mut Vec<ChessMove>) {
        self.generate_sliding_moves(square, &[9, 7, -9, -7], moves);
    }

    /// Generate rook moves.
    fn generate_rook_moves(&self, square: usize, moves: &mut Vec<ChessMove>) {
        self.generate_sliding_moves(square, &[8, -8, 1, -1], moves);
    }

    /// Generate queen moves (combining rook and bishop).
    fn generate_queen_moves(&self, square: usize, moves: &mut Vec<ChessMove>) {
        self.generate_sliding_moves(square, &[9, 7, -9, -7, 8, -8, 1, -1], moves);
    }

    /// Generate king moves.
    fn generate_king_moves(&self, square: usize, moves: &mut Vec<ChessMove>) {
        for &offset in &[9, 7, -9, -7, 8, -8, 1, -1] {
            let target = (square as isize + offset) as usize;
            if target < 64 && (!self.all_pieces.is_set(target) || self.is_opponent_piece(target, self.to_move)) {
                moves.push(ChessMove {
                    from: square,
                    to: target,
                    promotion: None,
                });
            }
        }
    
        // Add castling logic
        if self.can_castle_kingside(self.to_move) {
            let (king_from, king_to) = match self.to_move {
                PieceColour::White => (4, 6),
                PieceColour::Black => (60, 62),
            };
            if self.is_square_safe(king_from)
                && self.is_square_safe(king_from + 1)
                && self.is_square_safe(king_from + 2)
            {
                moves.push(ChessMove {
                    from: king_from,
                    to: king_to,
                    promotion: None,
                });
            }
        }
    
        if self.can_castle_queenside(self.to_move) {
            let (king_from, king_to) = match self.to_move {
                PieceColour::White => (4, 2),
                PieceColour::Black => (60, 58),
            };
            if self.is_square_safe(king_from)
                && self.is_square_safe(king_from - 1)
                && self.is_square_safe(king_from - 2)
            {
                moves.push(ChessMove {
                    from: king_from,
                    to: king_to,
                    promotion: None,
                });
            }
        }
    }
    
    

    /// Helper for sliding piece moves (bishop, rook, queen).
    fn generate_sliding_moves(&self, square: usize, directions: &[isize], moves: &mut Vec<ChessMove>) {
        for &direction in directions {
            let mut target = square as isize + direction;
            while target >= 0 && target < 64 {
                let target_usize = target as usize;
                if self.all_pieces.is_set(target_usize) {
                    if self.is_opponent_piece(target_usize, self.to_move) {
                        moves.push(ChessMove {
                            from: square,
                            to: target_usize,
                            promotion: None,
                        });
                    }
                    break;
                }
                moves.push(ChessMove {
                    from: square,
                    to: target_usize,
                    promotion: None,
                });
                target += direction;
            }
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BoardState;
    use crate::pieces::{PieceColour, PieceKind};
    use tracing_subscriber;

    fn init() {
        let _ = tracing_subscriber::fmt::try_init();
    }


    #[test]
    fn test_pawn_moves_white() {
        init();
        let mut board = BoardState::new();
        let moves = board.generate_moves();

        // Test single pawn move forward
        assert!(moves.contains(&ChessMove {
            from: 8, // a2
            to: 16,  // a3
            promotion: None,
        }));

        // Test double pawn move from starting position
        assert!(moves.contains(&ChessMove {
            from: 8, // a2
            to: 24,  // a4
            promotion: None,
        }));
    }

    #[test]
    fn test_knight_moves() {
        init();

        let mut board = BoardState::new();

        // Place a white knight at d4 (square 27)
        board.white_knights.set(27);
        board.all_white.set(27);
        board.all_pieces.set(27);

        tracing::info!("Set up board for knight at d4");

        let moves = board.generate_moves();

        // Log generated moves for knights
        for m in &moves {
            if m.from == 27 {
                tracing::debug!(?m, "Generated knight move");
            }
        }

        // Expected moves from d4
        let expected_moves = vec![
            ChessMove { from: 27, to: 44, promotion: None }, // f5
            ChessMove { from: 27, to: 42, promotion: None }, // e5
            ChessMove { from: 27, to: 37, promotion: None }, // c6
            ChessMove { from: 27, to: 33, promotion: None }, // c3
            ChessMove { from: 27, to: 17, promotion: None }, // b6
            ChessMove { from: 27, to: 21, promotion: None }, // b3
            ChessMove { from: 27, to: 12, promotion: None }, // e2
            ChessMove { from: 27, to: 10, promotion: None }, // f2
        ];

        // Check if all expected moves are in the generated moves
        for m in expected_moves {
            assert!(
                moves.contains(&m),
                "Missing expected move: {:?}, generated moves: {:?}",
                m,
                moves
            );
        }
    }

    #[test]
    fn test_pawn_moves_black() {
        init();
        let mut board = BoardState::new();
        board.to_move = PieceColour::Black;

        let moves = board.generate_moves();

        // Test single pawn move forward
        assert!(moves.contains(&ChessMove {
            from: 48, // a7
            to: 40,   // a6
            promotion: None,
        }));

        // Test double pawn move from starting position
        assert!(moves.contains(&ChessMove {
            from: 48, // a7
            to: 32,   // a5
            promotion: None,
        }));
    }

    #[test]
    fn test_initial_setup() {
        init();
        let board = BoardState::new();

        // Ensure pieces are in starting positions
        assert!(board.white_pawns.is_set(8)); // a2
        assert!(board.black_pawns.is_set(48)); // a7
        assert!(board.white_rooks.is_set(0)); // a1
        assert!(board.black_rooks.is_set(56)); // a8
        assert!(board.white_king.is_set(4)); // e1
        assert!(board.black_king.is_set(60)); // e8
    }

    #[test]
    fn test_castling_moves() {
        init();
        let mut board = BoardState::new();

        // Remove obstructing pieces
        board.all_pieces.clear(5); // f1
        board.all_pieces.clear(6); // g1
        board.all_pieces.clear(1); // b1
        board.all_pieces.clear(2); // c1
        board.all_pieces.clear(3); // d1

        // Set explicit castling rights
        board.update_castling_rights(true, true, true, true);

        let moves = board.generate_moves();

        // Validate kingside castling
        assert!(
            moves.contains(&ChessMove { from: 4, to: 6, promotion: None }),
            "Kingside castling move is missing"
        );

        // Validate queenside castling
        assert!(
            moves.contains(&ChessMove { from: 4, to: 2, promotion: None }),
            "Queenside castling move is missing"
        );
    }

    #[test]
    fn test_en_passant() {
        init();

        let mut board = BoardState::new();

        // Reset all bitboards to ensure a clean setup
        board.white_pawns.0 = 0;
        board.all_pieces.0 = 0;
        board.all_white.0 = 0;
        board.all_black.0 = 0;

        board.print_board();

        // Set up white pawn on e5
        board.white_pawns.set(36); // e5
        board.all_pieces.set(36);
        board.all_white.set(36);

        board.print_board();

        // Set up black pawn on d5 (moved from d7)
        board.black_pawns.set(35); // d5
        board.all_pieces.set(35);
        board.all_black.set(35);

        // Set en passant square
        board.en_passant_square = Some(43); // d6

        // Print the board for debugging
        board.print_board();

        // Generate moves for white
        board.to_move = PieceColour::White;
        let moves = board.generate_moves();

        // Check for en passant move
        assert!(moves.contains(&ChessMove {
            from: 36, // e5
            to: 43,  // d6 (en passant capture)
            promotion: None,
        }), "En passant capture is missing");
    }

    
}
