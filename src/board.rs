use crate::pieces::{Piece, PieceColour, PieceKind};
use crate::moves::ChessMove;
use crate::zorbist::ZobristHashing;
use std::ops::{BitAnd, BitAndAssign, BitOrAssign};

pub const BOARD_SIZE: usize = 8;
pub const TOTAL_SQUARES: usize = 64;

/// Represents a bitboard as a 64-bit integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn empty() -> Self {
        BitBoard(0)
    }

    pub fn set(&mut self, square: usize) {
        self.0 |= 1 << square;
        tracing::debug!("Set bitboard: {:064b} (square: {})", self.0, square);
    }

    pub fn clear(&mut self, square: usize) {
        self.0 &= !(1 << square);
    }

    pub fn is_set(&self, square: usize) -> bool {
        self.0 & (1 << square) != 0
    }

    pub fn print(&self) {
        for rank in (0..BOARD_SIZE).rev() {
            for file in 0..BOARD_SIZE {
                let square = rank * BOARD_SIZE + file;
                if self.is_set(square) {
                    print!("1 ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
        println!();
    }

}

impl BitAndAssign<u64> for BitBoard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs;
    }
}

impl BitAnd<u64> for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 & rhs)
    }
}

impl BitOrAssign<u64> for BitBoard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

/// Represents the entire chessboard using bitboards.
#[derive(Debug)]
pub struct BoardState {
    pub white_pawns: BitBoard,
    pub black_pawns: BitBoard,
    pub white_knights: BitBoard,
    pub black_knights: BitBoard,
    pub white_bishops: BitBoard,
    pub black_bishops: BitBoard,
    pub white_rooks: BitBoard,
    pub black_rooks: BitBoard,
    pub white_queens: BitBoard,
    pub black_queens: BitBoard,
    pub white_king: BitBoard,
    pub black_king: BitBoard,
    pub all_white: BitBoard,
    pub all_black: BitBoard,
    pub all_pieces: BitBoard,
    pub to_move: PieceColour,
    pub castling_rights: [bool; 4],
    pub en_passant_square: Option<usize>,
}

impl BoardState {
    pub fn new() -> Self {
        let mut board = BoardState {
            white_pawns: BitBoard::empty(),
            black_pawns: BitBoard::empty(),
            white_knights: BitBoard::empty(),
            black_knights: BitBoard::empty(),
            white_bishops: BitBoard::empty(),
            black_bishops: BitBoard::empty(),
            white_rooks: BitBoard::empty(),
            black_rooks: BitBoard::empty(),
            white_queens: BitBoard::empty(),
            black_queens: BitBoard::empty(),
            white_king: BitBoard::empty(),
            black_king: BitBoard::empty(),
            all_white: BitBoard::empty(),
            all_black: BitBoard::empty(),
            all_pieces: BitBoard::empty(),
            to_move: PieceColour::White,
            castling_rights: [true, true, true, true],
            en_passant_square: None,
        };

        board.setup_pieces();
        board
    }

    fn setup_pieces(&mut self) {
        // Set white pawns
        for i in 8..16 {
            self.white_pawns.set(i);
        }

        // Set black pawns
        for i in 48..56 {
            self.black_pawns.set(i);
        }

        // Set other pieces
        self.white_rooks.set(0);
        self.white_rooks.set(7);
        self.black_rooks.set(56);
        self.black_rooks.set(63);

        self.white_knights.set(1);
        self.white_knights.set(6);
        self.black_knights.set(57);
        self.black_knights.set(62);

        self.white_bishops.set(2);
        self.white_bishops.set(5);
        self.black_bishops.set(58);
        self.black_bishops.set(61);

        self.white_queens.set(3);
        self.black_queens.set(59);

        self.white_king.set(4);
        self.black_king.set(60);

        // Update aggregate bitboards
        self.update_aggregate_bitboards();
    }

    fn update_aggregate_bitboards(&mut self) {
        self.all_white = BitBoard(
            self.white_pawns.0
                | self.white_knights.0
                | self.white_bishops.0
                | self.white_rooks.0
                | self.white_queens.0
                | self.white_king.0,
        );

        self.all_black = BitBoard(
            self.black_pawns.0
                | self.black_knights.0
                | self.black_bishops.0
                | self.black_rooks.0
                | self.black_queens.0
                | self.black_king.0,
        );

        self.all_pieces = BitBoard(self.all_white.0 | self.all_black.0);
    }

    pub fn print_board(&self) {
        let mut squares = [". "; TOTAL_SQUARES];

        for i in 0..TOTAL_SQUARES {
            if self.white_pawns.is_set(i) {
                squares[i] = "P ";
            } else if self.black_pawns.is_set(i) {
                squares[i] = "p ";
            } else if self.white_knights.is_set(i) {
                squares[i] = "N ";
            } else if self.black_knights.is_set(i) {
                squares[i] = "n ";
            } else if self.white_bishops.is_set(i) {
                squares[i] = "B ";
            } else if self.black_bishops.is_set(i) {
                squares[i] = "b ";
            } else if self.white_rooks.is_set(i) {
                squares[i] = "R ";
            } else if self.black_rooks.is_set(i) {
                squares[i] = "r ";
            } else if self.white_queens.is_set(i) {
                squares[i] = "Q ";
            } else if self.black_queens.is_set(i) {
                squares[i] = "q ";
            } else if self.white_king.is_set(i) {
                squares[i] = "K ";
            } else if self.black_king.is_set(i) {
                squares[i] = "k ";
            }
        }

        println!("  a b c d e f g h");
        for rank in (0..BOARD_SIZE).rev() {
            print!("{} ", rank + 1);
            for file in 0..BOARD_SIZE {
                print!("{}", squares[rank * BOARD_SIZE + file]);
            }
            println!("");
        }
        println!("  a b c d e f g h");
    }

    pub fn piece_at(&self, square: usize) -> Option<crate::pieces::Piece> {
        if self.white_pawns.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::Pawn,
                colour: crate::pieces::PieceColour::White,
            })
        } else if self.black_pawns.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::Pawn,
                colour: crate::pieces::PieceColour::Black,
            })
        } else if self.white_knights.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::Knight,
                colour: crate::pieces::PieceColour::White,
            })
        } else if self.black_knights.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::Knight,
                colour: crate::pieces::PieceColour::Black,
            })
        } else if self.white_bishops.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::Bishop,
                colour: crate::pieces::PieceColour::White,
            })
        } else if self.black_bishops.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::Bishop,
                colour: crate::pieces::PieceColour::Black,
            })
        } else if self.white_rooks.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::Rook,
                colour: crate::pieces::PieceColour::White,
            })
        } else if self.black_rooks.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::Rook,
                colour: crate::pieces::PieceColour::Black,
            })
        } else if self.white_queens.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::Queen,
                colour: crate::pieces::PieceColour::White,
            })
        } else if self.black_queens.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::Queen,
                colour: crate::pieces::PieceColour::Black,
            })
        } else if self.white_king.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::King,
                colour: crate::pieces::PieceColour::White,
            })
        } else if self.black_king.is_set(square) {
            Some(crate::pieces::Piece {
                kind: crate::pieces::PieceKind::King,
                colour: crate::pieces::PieceColour::Black,
            })
        } else {
            None
        }
    }

    pub fn set_piece_at(&mut self, square: usize, piece: Piece) {
        let bit = 1u64 << square;

        // Clear the square on all bitboards
        self.clear_square(square);

        // Set the bit on the appropriate bitboard
        match (piece.colour, piece.kind) {
            (PieceColour::White, PieceKind::Pawn) => self.white_pawns |= bit,
            (PieceColour::Black, PieceKind::Pawn) => self.black_pawns |= bit,
            (PieceColour::White, PieceKind::Knight) => self.white_knights |= bit,
            (PieceColour::Black, PieceKind::Knight) => self.black_knights |= bit,
            (PieceColour::White, PieceKind::Bishop) => self.white_bishops |= bit,
            (PieceColour::Black, PieceKind::Bishop) => self.black_bishops |= bit,
            (PieceColour::White, PieceKind::Rook) => self.white_rooks |= bit,
            (PieceColour::Black, PieceKind::Rook) => self.black_rooks |= bit,
            (PieceColour::White, PieceKind::Queen) => self.white_queens |= bit,
            (PieceColour::Black, PieceKind::Queen) => self.black_queens |= bit,
            (PieceColour::White, PieceKind::King) => self.white_king |= bit,
            (PieceColour::Black, PieceKind::King) => self.black_king |= bit,
        }
    }

    pub fn update_castling_rights(&mut self, wk: bool, wq: bool, bk: bool, bq: bool) {
        self.castling_rights = [wk, wq, bk, bq];
    }

    /// Check if castling kingside is allowed for the current player.
    pub fn can_castle_kingside(&self, colour: PieceColour) -> bool {
        let (king_square, rook_square, empty_squares, check_squares) = match colour {
            PieceColour::White => (4, 7, [5, 6], [4, 5, 6]),
            PieceColour::Black => (60, 63, [61, 62], [60, 61, 62]),
        };

        let rights = match colour {
            PieceColour::White => self.castling_rights[0],
            PieceColour::Black => self.castling_rights[2],
        };

        rights
            && empty_squares.iter().all(|&sq| !self.all_pieces.is_set(sq))
            && check_squares.iter().all(|&sq| self.is_square_safe(sq))
            && self.validate_castling_pieces(king_square, rook_square)
    }

    /// Check if castling queenside is allowed for the current player.
    pub fn can_castle_queenside(&self, colour: PieceColour) -> bool {
        let (king_square, rook_square, empty_squares, check_squares) = match colour {
            PieceColour::White => (4, 0, [1, 2, 3], [2, 3, 4]),
            PieceColour::Black => (60, 56, [57, 58, 59], [58, 59, 60]),
        };
    
        let rights = match colour {
            PieceColour::White => self.castling_rights[1],
            PieceColour::Black => self.castling_rights[3],
        };
    
        rights
            && empty_squares.iter().all(|&sq| !self.all_pieces.is_set(sq))
            && check_squares.iter().all(|&sq| self.is_square_safe(sq))
            && self.validate_castling_pieces(king_square, rook_square)
    }

    /// Helper to check if king and rook are in the correct positions for castling.
    pub fn validate_castling_pieces(&self, king_square: usize, rook_square: usize) -> bool {
        self.piece_at(king_square).map_or(false, |piece| piece.kind == PieceKind::King)
            && self.piece_at(rook_square).map_or(false, |piece| piece.kind == PieceKind::Rook)
    }

    /// Generic method to validate castling conditions dynamically
    pub fn king_and_rook_can_castle(&self, king_square: usize, rook_square: usize, empty_squares: &[usize]) -> bool {
        self.is_square_safe(king_square)
            && self.is_square_safe(king_square + 1)
            && self.is_square_safe(king_square + 2)
            && self.all_pieces.is_set(rook_square) // Rook is present
            && empty_squares.iter().all(|&sq| !self.all_pieces.is_set(sq)) // Path is clear
    }

    
    pub fn is_square_safe(&self, square: usize) -> bool {
        // Check if the square is attacked by any opponent piece
        let opponent_colour = self.to_move.opposite();
    
        // Check pawn attacks
        let pawn_attack_offsets = if opponent_colour == PieceColour::White {
            [-9, -7]
        } else {
            [9, 7]
        };
        for &offset in &pawn_attack_offsets {
            let target = (square as isize + offset) as usize;
            if target < 64 {
                if let Some(piece) = self.piece_at(target) {
                    if piece.kind == PieceKind::Pawn && piece.colour == opponent_colour {
                        return false;
                    }
                }
            }
        }
    
        // Check knight attacks
        let knight_offsets = [17, 15, 10, 6, -17, -15, -10, -6];
        for &offset in &knight_offsets {
            let target = (square as isize + offset) as usize;
            if target < 64 {
                if let Some(piece) = self.piece_at(target) {
                    if piece.kind == PieceKind::Knight && piece.colour == opponent_colour {
                        return false;
                    }
                }
            }
        }
    
        // Check sliding piece attacks (bishop, rook, queen)
        let sliding_directions = &[9, 7, -9, -7, 8, -8, 1, -1];
        for &direction in sliding_directions {
            let mut target = square as isize + direction;
            while target >= 0 && target < 64 {
                let target_usize = target as usize;
                if let Some(piece) = self.piece_at(target_usize) {
                    if piece.colour == opponent_colour {
                        if (piece.kind == PieceKind::Bishop && [9, 7, -9, -7].contains(&direction))
                            || (piece.kind == PieceKind::Rook && [8, -8, 1, -1].contains(&direction))
                            || piece.kind == PieceKind::Queen
                        {
                            return false;
                        }
                    }
                    break;
                }
                target += direction;
            }
        }
    
        // Check king attacks
        let king_offsets = [9, 7, -9, -7, 8, -8, 1, -1];
        for &offset in &king_offsets {
            let target = (square as isize + offset) as usize;
            if target < 64 {
                if let Some(piece) = self.piece_at(target) {
                    if piece.kind == PieceKind::King && piece.colour == opponent_colour {
                        return false;
                    }
                }
            }
        }
    
        true
    }

    pub fn apply_move(&mut self, chess_move: ChessMove, zobrist: &mut ZobristHashing) {
        let from = chess_move.from;
        let to = chess_move.to;
    
        // Verify that the piece exists before attempting to move
        let piece = self.piece_at(from).expect("Piece must exist at 'from'");
    
        // Update en passant square before clearing 'from'
        self.update_en_passant_square(&chess_move);
    
        // Move the piece
        self.clear_square(from);
        self.set_piece_at(to, piece);
    
        // Handle special moves (e.g., en passant, promotion)
        if piece.kind == PieceKind::Pawn {
            if let Some(ep_square) = self.en_passant_square {
                if to == ep_square {
                    let captured_square = if piece.colour == PieceColour::White {
                        to - 8 // Black pawn behind
                    } else {
                        to + 8 // White pawn behind
                    };
                    self.clear_square(captured_square);
                }
            }
            if let Some(promotion) = chess_move.promotion {
                self.clear_square(to);
                self.set_piece_at(to, Piece {
                    kind: promotion,
                    colour: piece.colour,
                });
            }
        }
    
        // Flip the turn and update hash
        self.flip_turn();
        let new_hash = zobrist.compute_hash(self);
        tracing::debug!("Updated Zobrist hash: {}", new_hash);
    }
    

    fn get_piece_at_square(&self, square: usize) -> Option<Piece> {
        if self.white_pawns.is_set(square) {
            Some(Piece {
                kind: PieceKind::Pawn,
                colour: PieceColour::White,
            })
        } else if self.black_pawns.is_set(square) {
            Some(Piece {
                kind: PieceKind::Pawn,
                colour: PieceColour::Black,
            })
        } else if self.white_knights.is_set(square) {
            Some(Piece {
                kind: PieceKind::Knight,
                colour: PieceColour::White,
            })
        } else if self.black_knights.is_set(square) {
            Some(Piece {
                kind: PieceKind::Knight,
                colour: PieceColour::Black,
            })
        } else if self.white_bishops.is_set(square) {
            Some(Piece {
                kind: PieceKind::Bishop,
                colour: PieceColour::White,
            })
        } else if self.black_bishops.is_set(square) {
            Some(Piece {
                kind: PieceKind::Bishop,
                colour: PieceColour::Black,
            })
        } else if self.white_rooks.is_set(square) {
            Some(Piece {
                kind: PieceKind::Rook,
                colour: PieceColour::White,
            })
        } else if self.black_rooks.is_set(square) {
            Some(Piece {
                kind: PieceKind::Rook,
                colour: PieceColour::Black,
            })
        } else if self.white_queens.is_set(square) {
            Some(Piece {
                kind: PieceKind::Queen,
                colour: PieceColour::White,
            })
        } else if self.black_queens.is_set(square) {
            Some(Piece {
                kind: PieceKind::Queen,
                colour: PieceColour::Black,
            })
        } else if self.white_king.is_set(square) {
            Some(Piece {
                kind: PieceKind::King,
                colour: PieceColour::White,
            })
        } else if self.black_king.is_set(square) {
            Some(Piece {
                kind: PieceKind::King,
                colour: PieceColour::Black,
            })
        } else {
            None
        }
    }
    

    fn clear_square(&mut self, square: usize) {
        self.white_pawns.clear(square);
        self.black_pawns.clear(square);
        self.all_white.clear(square);
        self.all_black.clear(square);
        self.all_pieces.clear(square);
    }

    fn set_square(&mut self, square: usize, piece: Piece) {
        match piece.colour {
            PieceColour::White => {
                self.all_white.set(square);
                match piece.kind {
                    PieceKind::Pawn => self.white_pawns.set(square),
                    _ => { /* Set other piece types here */ }
                }
            }
            PieceColour::Black => {
                self.all_black.set(square);
                match piece.kind {
                    PieceKind::Pawn => self.black_pawns.set(square),
                    _ => { /* Set other piece types here */ }
                }
            }
        }
        self.all_pieces.set(square);
    }

    fn promote_pawn(&mut self, square: usize, promotion: PieceKind) {
        // Handle promotion by clearing the pawn and setting the promoted piece
        self.clear_square(square);
        self.set_square(square, Piece { kind: promotion, colour: self.to_move });
    }

    pub fn flip_turn(&mut self) {
        self.to_move = self.to_move.opposite();
    }

    fn update_en_passant_square(&mut self, chess_move: &ChessMove) {
        let from_rank = chess_move.from / 8;
        let to_rank = chess_move.to / 8;

        tracing::debug!(
            "update_en_passant_square: from={}, to={}, from_rank={}, to_rank={}",
            chess_move.from,
            chess_move.to,
            from_rank,
            to_rank
        );

        if let Some(piece) = self.piece_at(chess_move.from) {
            tracing::debug!("Piece at 'from': {:?}", piece);

            if piece.kind == PieceKind::Pawn && (to_rank as isize - from_rank as isize).abs() == 2 {
                self.en_passant_square = Some((chess_move.from + chess_move.to) / 2);
                tracing::debug!("En passant square set to: {:?}", self.en_passant_square);
                return;
            }
        } else {
            tracing::error!(
                "No piece found at 'from': {} during en passant update. Board state: {:?}",
                chess_move.from,
                self
            );
        }

        tracing::debug!("En passant square cleared");
        self.en_passant_square = None;
    }

    /// Validate en passant move legality.
    fn is_valid_en_passant(&self, from: usize, to: usize) -> bool {
        if let Some(ep_square) = self.en_passant_square {
            return to == ep_square;
        }
        false
    }

}

pub struct BitBoardIter {
    bitboard: BitBoard,
    index: usize,
}

impl Iterator for BitBoardIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 64 {
            if self.bitboard.is_set(self.index) {
                let result = self.index;
                self.index += 1;
                return Some(result);
            }
            self.index += 1;
        }
        None
    }
}

impl BitBoard {
    /// Returns an iterator over all set bits in the bitboard.
    pub fn iter(&self) -> BitBoardIter {
        BitBoardIter {
            bitboard: *self,
            index: 0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_board() {
        let board = BoardState::new();
        board.print_board();
        assert!(board.white_pawns.is_set(8)); // a2
        assert!(board.black_pawns.is_set(48)); // a7
    }

    #[test]
    fn test_bitboard_operations() {
        let mut bitboard = BitBoard::empty();
        bitboard.set(0); // Set a1
        assert!(bitboard.is_set(0));

        bitboard.set(63); // Set h8
        assert!(bitboard.is_set(63));

        bitboard.clear(0); // Clear a1
        assert!(!bitboard.is_set(0));
    }

    #[test]
    fn test_aggregate_bitboards() {
        let board = BoardState::new();
        assert!(board.all_white.is_set(8)); // a2
        assert!(board.all_black.is_set(48)); // a7
        assert!(board.all_pieces.is_set(4)); // e1
    }

    #[test]
    fn test_piece_representation() {
        let board = BoardState::new();
        assert!(board.white_king.is_set(4)); // e1
        assert!(board.black_king.is_set(60)); // e8
    }

    #[test]
    fn test_en_passant_generation() {
        let mut board = BoardState::new();
        let mut zobrist = ZobristHashing::new();

        tracing::debug!("Setting up test board state");
        board.black_pawns.set(51); // d7
        board.all_pieces.set(51);

        tracing::debug!("Board state before move: {:?}", board);

        let chess_move = ChessMove {
            from: 51, // d7
            to: 35,   // d5
            promotion: None,
        };

        board.apply_move(chess_move, &mut zobrist);

        tracing::debug!(
            "En passant square after move: {:?}, Board state: {:?}",
            board.en_passant_square,
            board
        );

        assert_eq!(
            board.en_passant_square,
            Some(43),
            "En passant square should be 43"
        );
    }


    #[test]
    fn test_update_en_passant_square() {
        let mut board = BoardState::new();
    
        tracing::debug!("Setting up test board state for en passant");
        board.black_pawns.set(51); // d7
        board.all_pieces.set(51);
    
        let chess_move = ChessMove {
            from: 51, // d7
            to: 35,   // d5
            promotion: None,
        };
    
        tracing::debug!("Applying update_en_passant_square");
        board.update_en_passant_square(&chess_move);
    
        assert_eq!(
            board.en_passant_square,
            Some(43),
            "En passant square should be 43"
        );
    }
    
    #[test]
    fn test_board_state_before_en_passant() {
        let mut board = BoardState::new();
    
        board.black_pawns.set(51); // d7
        board.all_pieces.set(51);
    
        assert!(board.black_pawns.is_set(51), "Black pawn should be on d7");
        assert!(board.all_pieces.is_set(51), "All pieces should include pawn on d7");
    
        let chess_move = ChessMove {
            from: 51, // d7
            to: 35,   // d5
            promotion: None,
        };
    
        board.apply_move(chess_move, &mut ZobristHashing::new());
    
        assert_eq!(
            board.en_passant_square,
            Some(43),
            "En passant square should be set after two-square pawn move"
        );
    }
    

    #[test]
    fn test_castling_rights() {
        let mut board = BoardState::new();

        // Kingside castling setup
        board.all_pieces.clear(5); // f1
        board.all_pieces.clear(6); // g1
        assert!(board.can_castle_kingside(PieceColour::White));

        // Queenside castling setup
        board.all_pieces.clear(1); // b1
        board.all_pieces.clear(2); // c1
        board.all_pieces.clear(3); // d1
        assert!(board.can_castle_queenside(PieceColour::White));
    }


    #[test]
    fn test_castling_kingside_under_attack() {
        let mut board = BoardState::new();

        // Clear squares for kingside castling
        board.all_pieces.clear(5); // f1
        board.all_pieces.clear(6); // g1

        // Place an opposing rook attacking f1
        board.set_piece_at(37, Piece { kind: PieceKind::Rook, colour: PieceColour::Black });

        assert!(!board.can_castle_kingside(PieceColour::White), "Should not allow kingside castling if f1 is under attack");
    }

    #[test]
    fn test_castling_queenside_under_attack() {
        let mut board = BoardState::new();

        // Clear squares for queenside castling
        board.all_pieces.clear(1); // b1
        board.all_pieces.clear(2); // c1
        board.all_pieces.clear(3); // d1

        // Place an opposing bishop attacking c1
        board.set_piece_at(42, Piece { kind: PieceKind::Bishop, colour: PieceColour::Black });

        assert!(!board.can_castle_queenside(PieceColour::White), "Should not allow queenside castling if c1 is under attack");
    }


}
