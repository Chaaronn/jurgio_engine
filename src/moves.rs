use crate::board::{BoardState, Square};
use crate::board::BOARD_SIZE;
use crate::pieces::{Piece, PieceKind, PieceColour};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ChessMove {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub promotion: Option<PieceKind>, // Optional promotion for pawns.
}

// Placeholder for now
impl BoardState {
    /// Applies a move to the board. Returns `Err` if the move is invalid.
    pub fn apply_move(&mut self, chess_move: ChessMove) -> Result<(), &'static str> {
        
        let (from_row, from_col) = chess_move.from;
        let (to_row, to_col) = chess_move.to;

        // Get the piece from starting square
        if let Square::Full(piece) = self.board[from_row][from_col] {
            
            // Ensure not moving to the same square
            if from_row == to_row && from_col == to_col {
                return Err("Invalid move: Source and destination are the same");
            }

            // Move piece

            self.board[to_row][to_col] = Square::Full(piece);
            self.board[from_row][from_col] = Square::Empty;

            // Handle promotion (if applicable).
            if let Some(promotion_kind) = chess_move.promotion {
                if piece.kind == PieceKind::Pawn
                    && ((piece.colour == PieceColour::White && to_row == 0)
                        || (piece.colour == PieceColour::Black && to_row == BOARD_SIZE - 1))
                {
                    self.board[to_row][to_col] = Square::Full(Piece {
                        kind: promotion_kind,
                        colour: piece.colour,
                    });
                } else {
                    return Err("Invalid promotion: Only pawns on the last rank can promote");
                }
            }

            // Swap turns after the move.
            self.to_move = self.to_move.opposite();

            Ok(())
        } else {
            Err("Invalid move: No piece at the source square")
        }
    }



    /// Generates all valid moves for a piece at a given position.
    pub fn valid_moves(&self, row: usize, col: usize) -> Vec<ChessMove> {
        
        let mut moves = Vec::new();

        // Get the piece at the given position.
        if let Square::Full(piece) = self.board[row][col] {
            if piece.colour != self.to_move {
                return moves; // Return an empty list if it's not the current player's turn.
            }

            match piece.kind {
                PieceKind::Pawn => self.pawn_moves(row, col, piece, &mut moves),
                PieceKind::Rook => self.rook_moves(row, col, piece, &mut moves),
                PieceKind::Knight => self.knight_moves(row, col, piece, &mut moves),
                PieceKind::Bishop => self.bishop_moves(row, col, piece, &mut moves),
                PieceKind::Queen => {
                    self.rook_moves(row, col, piece, &mut moves);
                    self.bishop_moves(row, col, piece, &mut moves);
                }
                PieceKind::King => self.king_moves(row, col, piece, &mut moves),
            }
        }

        moves
    }

    /// Adds valid pawn moves to the moves list.
    fn pawn_moves(
        &self,
        row: usize,
        col: usize,
        piece: Piece,
        moves: &mut Vec<ChessMove>,
    ) {
        // Add basic pawn movement logic (one step forward, captures, en passant, promotion).
        // For example:
        let forward = if piece.colour == PieceColour::White { -1 } else { 1 };

        // Forward move.
        if row as isize + forward >= 0 && row as isize + forward < BOARD_SIZE as isize {
            let target_row = (row as isize + forward) as usize;
            if let Square::Empty = self.board[target_row][col] {
                moves.push(ChessMove {
                    from: (row, col),
                    to: (target_row, col),
                    promotion: None,
                });

                // Double move for pawns on their initial rank.
                if (piece.colour == PieceColour::White && row == 6)
                    || (piece.colour == PieceColour::Black && row == 1)
                {
                    let double_row = (row as isize + forward * 2) as usize;
                    if let Square::Empty = self.board[double_row][col] {
                        moves.push(ChessMove {
                            from: (row, col),
                            to: (double_row, col),
                            promotion: None,
                        });
                    }
                }
            }
        }

        // Add capture logic, promotion, etc. here
    }

    /// Adds valid rook moves to the moves list.
    fn rook_moves(
        &self,
        row: usize,
        col: usize,
        piece: Piece,
        moves: &mut Vec<ChessMove>,
    ) {
        // Add basic  movement logic 
    }

    /// Adds valid bishop moves to the moves list.
    fn bishop_moves(
        &self,
        row: usize,
        col: usize,
        piece: Piece,
        moves: &mut Vec<ChessMove>,
    ) {
        // Add basic movement logic 
    }

    /// Adds valid knight moves to the moves list.
    fn knight_moves(
        &self,
        row: usize,
        col: usize,
        piece: Piece,
        moves: &mut Vec<ChessMove>,
    ) {
        // Add basic movement logic 
    }

    /// Adds valid king moves to the moves list.
    fn king_moves(
        &self,
        row: usize,
        col: usize,
        piece: Piece,
        moves: &mut Vec<ChessMove>,
    ) {
        // Add basic movement logic 
    }
    
}