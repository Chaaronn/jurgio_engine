use crate::board::{BoardState, Square};
use crate::board::BOARD_SIZE;
use crate::pieces::{Piece, PieceKind, PieceColour};
use tracing::{info, span, Level, debug, error};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ChessMove {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub promotion: Option<PieceKind>, // Optional promotion for pawns.
}

impl BoardState {
    /// Applies a move to the board. Returns `Err` if the move is invalid.
    pub fn apply_move(&mut self, chess_move: ChessMove, last_move: Option<ChessMove>) -> Result<(), String> {
        let (from_row, from_col) = chess_move.from;
        let (to_row, to_col) = chess_move.to;
    
        debug!(
            "Attempting to apply move: {:?} with last move: {:?}",
            chess_move, last_move
        );
        debug!("Last move passed to valid_moves: {:?}", last_move);
    
        // Validate move
        let valid_moves = self.valid_moves(from_row, from_col, last_move);
        debug!("Generated valid moves for ({}, {}): {:?}", from_row, from_col, valid_moves);
    
        if !valid_moves.iter().any(|m| m.to == (to_row, to_col) && m.promotion == chess_move.promotion) {
            error!(
                "Move validation failed: {:?}. Valid moves: {:?}",
                chess_move, valid_moves
            );
            return Err("Invalid move".to_string());
        }
    
        // Move the piece
        let moving_piece = self.board[from_row][from_col];
        debug!(
            "Moving piece {:?} from ({}, {}) to ({}, {})",
            moving_piece, from_row, from_col, to_row, to_col
        );
        self.board[to_row][to_col] = moving_piece;
        self.clear_square(from_row, from_col);
    
        // Handle en passant
        if let Some(last_move) = last_move {
            let (last_to_row, last_to_col) = last_move.to;
        
            // Check if the current move is an en passant capture
            if let Square::Full(piece) = moving_piece {
                if piece.kind == PieceKind::Pawn
                    && (from_row as isize - to_row as isize).abs() == 1
                    && (from_col as isize - to_col as isize).abs() == 1
                    && self.board[to_row][to_col] == Square::Empty
                {
                    // Ensure last move was a double move by the opponent's pawn
                    if let Square::Full(last_piece) = self.board[last_to_row][last_to_col] {
                        if last_piece.kind == PieceKind::Pawn
                            && last_piece.colour != piece.colour
                            && (last_to_row as isize - last_move.from.0 as isize).abs() == 2
                            && last_to_col == to_col
                        {
                            let captured_pawn_row = if piece.colour == PieceColour::White {
                                to_row + 1
                            } else {
                                to_row - 1
                            };
                            debug!(
                                "En passant capture: clearing captured pawn at ({}, {}).",
                                captured_pawn_row, to_col
                            );
                            self.clear_square(captured_pawn_row, to_col);
                        }
                    }
                }
            }
        }

    
        // Handle promotion
        if let Some(promotion_kind) = chess_move.promotion {
            debug!(
                "Handling promotion for move: {:?}, promoting to {:?}",
                chess_move, promotion_kind
            );
    
            if to_row == 0 || to_row == BOARD_SIZE - 1 {
                if let Square::Full(piece) = self.board[to_row][to_col] {
                    if piece.kind == PieceKind::Pawn {
                        self.board[to_row][to_col] = Square::Full(Piece {
                            kind: promotion_kind,
                            colour: piece.colour,
                        });
                        debug!(
                            "Promotion applied: Pawn at ({}, {}) promoted to {:?}",
                            to_row, to_col, promotion_kind
                        );
                    } else {
                        return Err("Only pawns can be promoted".to_string());
                    }
                }
            } else {
                return Err("Promotion only allowed on the last rank".to_string());
            }
        }
    
        debug!(
            "Move applied successfully: {:?}, board state after move:\n{:?}",
            chess_move, self.board
        );
    
        // Update turn
        self.switch_turn();
        debug!("Turn updated. Next turn: {:?}", self.to_move);
        Ok(())
    }
    
    
    /// Generates all valid moves for a piece at a given position.
    pub fn valid_moves(&self, row: usize, col: usize, last_move: Option<ChessMove>) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        debug!(
            "Valid moves calculation started for piece at ({}, {}). Last move: {:?}",
            row, col, last_move
        );
    
        if let Square::Full(piece) = self.board[row][col] {
            if piece.colour != self.to_move {
                debug!(
                    "Piece at ({}, {}) does not match current turn colour {:?}. No moves generated.",
                    row, col, self.to_move
                );
                return moves;
            }

            if piece.kind == PieceKind::Pawn {
                let direction = if piece.colour == PieceColour::White { -1 } else { 1 };
                if let Some(last_move) = last_move {
                    let (last_from_row, _last_from_col) = last_move.from;
                    let (last_to_row, last_to_col) = last_move.to;
            
                    // Fetch the piece at the destination of the last move
                    if let Square::Full(last_piece) = self.board[last_to_row][last_to_col] {
                        if last_from_row as isize - last_to_row as isize == 2 * direction
                            && last_piece.kind == PieceKind::Pawn
                            && (last_to_col as isize - col as isize).abs() == 1
                        {
                            // Add en passant move
                            moves.push(ChessMove {
                                from: (row, col),
                                to: ((last_to_row as isize + direction) as usize, last_to_col),
                                promotion: None,
                            });
                            debug!(
                                "En passant move added for pawn at ({}, {}) to ({}, {}).",
                                row, col, (last_to_row as isize + direction) as usize, last_to_col
                            );
                        }
                    }
                }
            }
    
            match piece.kind {
                PieceKind::Pawn => {
                    debug!("Calculating pawn moves for piece at ({}, {})", row, col);
                    self.pawn_moves(row, col, piece, &mut moves, last_move);
                }
                PieceKind::Rook => self.rook_moves(row, col, piece, &mut moves),
                PieceKind::Knight => self.knight_moves(row, col, piece, &mut moves),
                PieceKind::Bishop => self.bishop_moves(row, col, piece, &mut moves),
                PieceKind::Queen => {
                    self.rook_moves(row, col, piece, &mut moves);
                    self.bishop_moves(row, col, piece, &mut moves);
                }
                PieceKind::King => self.king_moves(row, col, piece, &mut moves),                
            }
        } else {
            debug!(
                "No piece found at ({}, {}). No moves generated.",
                row, col
            );
        }
    
        debug!("Valid moves for ({}, {}): {:?}", row, col, moves);
        moves
    }
    
    

    /// Adds valid pawn moves to the moves list.
    fn pawn_moves(
        &self,
        row: usize,
        col: usize,
        piece: Piece,
        moves: &mut Vec<ChessMove>,
        last_move: Option<ChessMove>,
    ) {
        let forward = if piece.colour == PieceColour::White { -1 } else { 1 };

        // Forward move.
        if row as isize + forward >= 0 && row as isize + forward < BOARD_SIZE as isize {
            let target_row = (row as isize + forward) as usize;
            debug!(
                "Checking forward for pawn at ({}, {}): target_row = {}, target_col = {}",
                row, col, target_row, col
            );

            // promotion for forward moves
            if let Square::Empty = self.board[target_row][col] {
                
                debug!("Pawn ({:?}) at ({}, {}) considers forward move to ({}, {})", piece.colour, row, col, target_row, col);

                if (piece.colour == PieceColour::White && target_row == 0)
                    || (piece.colour == PieceColour::Black && target_row == BOARD_SIZE - 1)
                {
                    debug!(
                        "Adding promotion moves for pawn at ({}, {}): target_row = {}",
                        row, col, target_row
                    );
                    // Loop through possible promotion pieces to add
                    for &promotion_kind in &[PieceKind::Queen, PieceKind::Rook, PieceKind::Bishop, PieceKind::Knight] {
                        
                        debug!(
                            "Adding promotion move for pawn at ({}, {}): target_row = {}, promotion_kind = {:?}",
                            row, col, target_row, promotion_kind
                        );
                        
                        moves.push(ChessMove {
                            from: (row, col),
                            to: (target_row, col),
                            promotion: Some(promotion_kind),
                        });
                    }
                } else {
                    // No promotion moves, so add the normal one
                    debug!("Pawn at ({}, {}) adds forward move to ({}, {})", row, col, target_row, col);
                    moves.push(ChessMove {
                        from: (row, col),
                        to: (target_row, col),
                        promotion: None,
                    });
                }

                // Double move from starting rank.
                let starting_rank = if piece.colour == PieceColour::White { 6 } else { 1 };
                if row == starting_rank {
                    
                    let double_row = (row as isize + forward * 2) as usize;
                    if let Square::Empty = self.board[double_row][col] {
                        debug!("Pawn at ({}, {}) adds double move to ({}, {})", row, col, double_row, col);
                        moves.push(ChessMove {
                            from: (row, col),
                            to: (double_row, col),
                            promotion: None,
                        });
                    }
                }
            } else {
                debug!("Pawn at ({}, {}) cannot move forward, square ({}, {}) is occupied", row, col, target_row, col);
            }
        }

        // Captures.
        for &col_offset in &[-1, 1] {
            if col as isize + col_offset >= 0 && col as isize + col_offset < BOARD_SIZE as isize {
                let target_col = (col as isize + col_offset) as usize;
                let target_row = (row as isize + forward) as usize;

                if let Square::Full(target_piece) = self.board[target_row][target_col] {
                    if target_piece.colour != piece.colour {
                        debug!(
                            "Pawn at ({}, {}) captures opponent's piece at ({}, {})",
                            row, col, target_row, target_col
                        );

                        // promotion
                        if (piece.colour == PieceColour::White && target_row == 0)
                            || (piece.colour == PieceColour::Black && target_row == BOARD_SIZE - 1)
                        {
                            for &promotion_kind in &[PieceKind::Queen, PieceKind::Rook, PieceKind::Bishop, PieceKind::Knight] {
                                moves.push(ChessMove {
                                    from: (row, col),
                                    to: (target_row, col),
                                    promotion: Some(promotion_kind),
                                });
                            }
                        } else {
                            debug!("Pawn at ({}, {}) adds capture move to ({}, {})", row, col, target_row, target_col);
                            moves.push(ChessMove {
                                from: (row, col),
                                to: (target_row, target_col),
                                promotion: None,
                            });
                        }
                    }
                } else {
                    debug!("Pawn at ({}, {}) cannot capture at ({}, {}), square is empty", row, col, target_row, target_col);
                }

                // Check for en passant
                if let Some(last_move) = last_move {
                    if let Square::Full(last_piece) = self.board[last_move.to.0][last_move.to.1] {
                        if last_piece.kind == PieceKind::Pawn

                            && last_piece.colour != piece.colour
                            && (last_move.from.0 as isize - last_move.to.0 as isize).abs() == 2
                            && (last_move.to.1 as isize - col as isize).abs() == 1
                        {
                            let en_passant_row = (row as isize + forward) as usize;
                            moves.push(ChessMove {
                                from: (row, col),
                                to: (en_passant_row, last_move.to.1),
                                promotion: None,
                            });
                        }
                    }
                }
            }
        }
    }


    /// Adds valid rook moves to the moves list.
    fn rook_moves(
        &self,
        row: usize,
        col: usize,
        piece: Piece,
        moves: &mut Vec<ChessMove>,
    ) {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]; // Up, down, right, left
    
        for (dr, dc) in directions {
            let mut r = row as isize;
            let mut c = col as isize;
    
            debug!("Rook starting direction dr: {}, dc: {}", dr, dc);
    
            loop {
                r += dr;
                c += dc;
    
                if r < 0 || r >= BOARD_SIZE as isize || c < 0 || c >= BOARD_SIZE as isize {
                    debug!("Rook out of bounds at r: {}, c: {}", r, c);
                    break; // Out of bounds
                }
    
                let target_row = r as usize;
                let target_col = c as usize;
    
                debug!("Rook considering move to ({}, {})", target_row, target_col);
    
                match self.board[target_row][target_col] {
                    Square::Empty => {
                        debug!("Square ({}, {}) is empty, adding Rook move", target_row, target_col);
                        moves.push(ChessMove {
                            from: (row, col),
                            to: (target_row, target_col),
                            promotion: None,
                        });
                    }
                    Square::Full(target_piece) => {
                        if target_piece.colour != piece.colour {
                            debug!(
                                "Square ({}, {}) has opponent's piece, adding Rook capture move",
                                target_row, target_col
                            );
                            moves.push(ChessMove {
                                from: (row, col),
                                to: (target_row, target_col),
                                promotion: None,
                            });
                        } else {
                            debug!(
                                "Square ({}, {}) has friendly piece, Rook stopping in this direction",
                                target_row, target_col
                            );
                        }
                        break; // Stop if a piece is encountered
                    }
                }
            }
        }
    }
    

    /// Adds valid bishop moves to the moves list.
    fn bishop_moves(
        &self,
        row: usize,
        col: usize,
        piece: Piece,
        moves: &mut Vec<ChessMove>,
    ) {
        let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    
        for (dr, dc) in directions {
            let mut r = row as isize;
            let mut c = col as isize;
    
            debug!(
                "Bishop starting from ({}, {}) in direction ({}, {})",
                row, col, dr, dc
            );
    
            loop {
                r += dr;
                c += dc;
    
                if r < 0 || r >= BOARD_SIZE as isize || c < 0 || c >= BOARD_SIZE as isize {
                    debug!("Out of bounds at r: {}, c: {}", r, c);
                    break;
                }
    
                let target_row = r as usize;
                let target_col = c as usize;
    
                debug!(
                    "Bishop considering move to ({}, {}), direction ({}, {})",
                    target_row, target_col, dr, dc
                );
    
                match self.board[target_row][target_col] {
                    Square::Empty => {
                        debug!("Square ({}, {}) is empty, adding Bishop move", target_row, target_col);
                        moves.push(ChessMove {
                            from: (row, col),
                            to: (target_row, target_col),
                            promotion: None,
                        });
                    }
                    Square::Full(target_piece) => {
                        if target_piece.colour != piece.colour {
                            debug!(
                                "Square ({}, {}) has opponent's piece, adding Bishop capture move",
                                target_row, target_col
                            );
                            moves.push(ChessMove {
                                from: (row, col),
                                to: (target_row, target_col),
                                promotion: None,
                            });
                        } else {
                            debug!(
                                "Square ({}, {}) has friendly piece, Bishop stopping in this direction",
                                target_row, target_col
                            );
                        }
                        break; // Stop at the first occupied square
                    }
                }
            }
        }
    }

    /// Adds valid knight moves to the moves list.
    fn knight_moves(
        &self,
        row: usize,
        col: usize,
        piece: Piece,
        moves: &mut Vec<ChessMove>,
    ) {
        let moveset = [
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2),
        ];

        for (dr, dc) in moveset {
            let r = row as isize + dr;
            let c = col as isize + dc;

            if r >= 0 && r < BOARD_SIZE as isize && c >= 0 && c < BOARD_SIZE as isize {
                let target_row = r as usize;
                let target_col = c as usize;

                match self.board[target_row][target_col] {
                    Square::Empty => moves.push(ChessMove {
                        from: (row, col),
                        to: (target_row, target_col),
                        promotion: None,
                    }),
                    Square::Full(target_piece) => {
                        if target_piece.colour != piece.colour {
                            moves.push(ChessMove {
                                from: (row, col),
                                to: (target_row, target_col),
                                promotion: None,
                            });
                        }
                    }
                }
            }
        }
    }

    /// Adds valid king moves to the moves list.
    fn king_moves(
        &self,
        row: usize,
        col: usize,
        piece: Piece,
        moves: &mut Vec<ChessMove>,
    ) {
        let moveset = [
            (1, 0), (-1, 0), (0, 1), (0, -1), // Cardinal directions.
            (1, 1), (1, -1), (-1, 1), (-1, -1), // Diagonals.
        ];
    
        debug!("Calculating king moves for King at ({}, {})", row, col);
    
        for (dr, dc) in moveset {
            let r = row as isize + dr;
            let c = col as isize + dc;
    
            if r >= 0 && r < BOARD_SIZE as isize && c >= 0 && c < BOARD_SIZE as isize {
                let target_row = r as usize;
                let target_col = c as usize;
    
                debug!("Evaluating move to ({}, {})", target_row, target_col);
    
                match self.board[target_row][target_col] {
                    Square::Empty => {
                        debug!("Square ({}, {}) is empty, adding King move", target_row, target_col);
                        moves.push(ChessMove {
                            from: (row, col),
                            to: (target_row, target_col),
                            promotion: None,
                        });
                    }
                    Square::Full(target_piece) => {
                        if target_piece.colour != piece.colour {
                            debug!(
                                "Square ({}, {}) has opponent's piece, adding King capture move",
                                target_row, target_col
                            );
                            moves.push(ChessMove {
                                from: (row, col),
                                to: (target_row, target_col),
                                promotion: None,
                            });
                        } else {
                            debug!(
                                "Square ({}, {}) has friendly piece, cannot move there",
                                target_row, target_col
                            );
                        }
                    }
                }
            } else {
                debug!("Move to ({}, {}) is out of bounds", r, c);
            }
        }
    }
    
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber;

    fn init() {
        let _ = tracing_subscriber::fmt::try_init();
    }

    #[test]
    fn test_apply_move_basic() {
        init();
        let mut board = BoardState::new();

        // Move a white pawn from e2 to e4.
        let chess_move = ChessMove {
            from: (6, 4), // e2
            to: (4, 4),   // e4
            promotion: None,
        };

        assert!(board.apply_move(chess_move, None).is_ok());
        assert!(matches!(board.board[4][4], Square::Full(Piece { kind: PieceKind::Pawn, colour: PieceColour::White })));
        assert!(matches!(board.board[6][4], Square::Empty));
    }

    #[test]
    fn test_apply_move_invalid_source() {
        init();
        let mut board = BoardState::new();

        // Try to move a piece from an empty square.
        let chess_move = ChessMove {
            from: (4, 4), // Empty square
            to: (3, 4),
            promotion: None,
        };

        assert!(board.apply_move(chess_move, None).is_err());
    }

    #[test]
    fn test_pawn_double_move() {
        init();
        let mut board = BoardState::new();

        // White pawn double move from e2 to e4.
        let chess_move = ChessMove {
            from: (6, 4), // e2
            to: (4, 4),   // e4
            promotion: None,
        };

        assert!(board.apply_move(chess_move, None).is_ok());
        assert!(matches!(board.board[4][4], Square::Full(Piece { kind: PieceKind::Pawn, colour: PieceColour::White })));
        assert!(matches!(board.board[6][4], Square::Empty));
    }

    #[test]
    fn test_pawn_promotion() {
        init();
        let mut board = BoardState::new();

        // Place a white pawn on e7.
        board.board[1][4] = Square::Full(Piece {
            kind: PieceKind::Pawn,
            colour: PieceColour::White,
        });

        // Promote the pawn to a queen.
        let chess_move = ChessMove {
            from: (1, 4), // e7
            to: (0, 4),   // e8
            promotion: Some(PieceKind::Queen),
        };

        assert!(board.apply_move(chess_move, None).is_ok());
        assert!(matches!(board.board[0][4], Square::Full(Piece { kind: PieceKind::Queen, colour: PieceColour::White })));

        // Invalid promotion: no promotion specified
        let invalid_promotion = ChessMove {
            from: (1, 4),
            to: (0, 4),
            promotion: None,
        };
        assert!(board.apply_move(invalid_promotion, None).is_err());
    }



    #[test]
    fn test_pawn_capture() {
        init();
        let mut board = BoardState::new();

        // Place a white pawn on e4 and a black pawn on d5.
        board.board[4][4] = Square::Full(Piece {
            kind: PieceKind::Pawn,
            colour: PieceColour::White,
        });
        board.board[3][3] = Square::Full(Piece {
            kind: PieceKind::Pawn,
            colour: PieceColour::Black,
        });

        // White pawn captures black pawn.
        let chess_move = ChessMove {
            from: (4, 4), // e4
            to: (3, 3),   // d5
            promotion: None,
        };

        assert!(board.apply_move(chess_move, None).is_ok());
        assert!(matches!(board.board[3][3], Square::Full(Piece { kind: PieceKind::Pawn, colour: PieceColour::White })));
        assert!(matches!(board.board[4][4], Square::Empty));
    }

    #[test]
    fn test_en_passant() {
        init();
        let mut board = BoardState::new();
    
        // Set up en passant scenario.
        board.board[4][4] = Square::Full(Piece {
            kind: PieceKind::Pawn,
            colour: PieceColour::White,
        });
        board.board[6][3] = Square::Full(Piece {
            kind: PieceKind::Pawn,
            colour: PieceColour::Black,
        });
    
        // Black pawn performs a double move.
        let black_double_move = ChessMove {
            from: (6, 3),
            to: (4, 3),
            promotion: None,
        };
        assert!(board.apply_move(black_double_move, None).is_ok());
        assert!(matches!(
            board.board[4][3],
            Square::Full(Piece { kind: PieceKind::Pawn, colour: PieceColour::Black })
        ));
    
        // Verify en passant is valid.
        let valid_moves = board.valid_moves(4, 4, Some(black_double_move));
        assert!(valid_moves.iter().any(|m| m.to == (5, 3)), "En passant move not generated.");
    
        // Perform en passant capture.
        let en_passant_move = ChessMove {
            from: (4, 4),
            to: (5, 3),
            promotion: None,
        };
        assert!(board.apply_move(en_passant_move, Some(black_double_move)).is_ok());
        assert!(matches!(
            board.board[5][3],
            Square::Full(Piece { kind: PieceKind::Pawn, colour: PieceColour::White })
        ));
        assert!(matches!(board.board[4][3], Square::Empty));
        assert!(matches!(board.board[4][3], Square::Full(Piece { kind: PieceKind::Pawn, colour: PieceColour::Black })));

    }
    





    #[test]
    fn test_rook_moves() {
        init();
        let mut board = BoardState::new();

        // Place a white rook on a1.
        board.board[7][0] = Square::Full(Piece {
            kind: PieceKind::Rook,
            colour: PieceColour::White,
        });

        // Clear surrounding squares to test movement.
        for row in 0..7 {
            board.board[row][0] = Square::Empty; // Clear the file above the rook
        }
        for col in 1..8 {
            board.board[7][col] = Square::Empty; // Clear the rank to the right
        }

        let moves = board.valid_moves(7, 0, None);
        
        // Rook should move to all squares along the rank and file.
        assert!(moves.iter().any(|m| m.to == (6, 0))); // Move up
        assert!(moves.iter().any(|m| m.to == (7, 7))); // Move right
        assert!(moves.iter().any(|m| m.to == (7, 1))); // Move along rank
        assert!(moves.iter().any(|m| m.to == (0, 0))); // Move to top of file
    }

    #[test]
    fn test_knight_moves() {
        init();
        let mut board = BoardState::new();

        // Place a white knight on b1.
        board.board[7][1] = Square::Full(Piece {
            kind: PieceKind::Knight,
            colour: PieceColour::White,
        });

        let moves = board.valid_moves(7, 1, None);
        assert!(moves.iter().any(|m| m.to == (5, 0))); // Knight jump.
        assert!(moves.iter().any(|m| m.to == (5, 2))); // Another knight jump.
    }

    #[test]
    fn test_bishop_moves() {
        init();
        let mut board = BoardState::new();

        // Clear the board for the test.
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                board.board[row][col] = Square::Empty;
            }
        }

        // Place a white bishop on c1.
        board.board[7][2] = Square::Full(Piece {
            kind: PieceKind::Bishop,
            colour: PieceColour::White,
        });

        let moves = board.valid_moves(7, 2, None);

        // Ensure the bishop can move diagonally to the expected squares.
        assert!(moves.iter().any(|m| m.to == (4, 5))); // Diagonal movement.
        assert!(moves.iter().any(|m| m.to == (3, 6))); // Another diagonal.
    }


    #[test]
    fn test_king_moves() {
        init();
        let mut board = BoardState::new();

        // Place a white king on e1.
        board.board[7][4] = Square::Full(Piece {
            kind: PieceKind::King,
            colour: PieceColour::White,
        });

        // Confirm the square in front of the King is occupied by a Pawn.
        board.board[6][4] = Square::Full(Piece {
            kind: PieceKind::Pawn,
            colour: PieceColour::White,
        });

        println!("Board state: {:?}", board.board);

        let moves = board.valid_moves(7, 4, None);
        println!("Generated moves: {:?}", moves);

        // Assert the King cannot move to (6, 4).
        assert!(!moves.iter().any(|m| m.to == (6, 4)));
    }
}
