use crate::pieces::{Piece, PieceKind, PieceColour};
use tracing::{info, span, Level, debug};

pub const BOARD_SIZE: usize = 8;

/// Represents a single square on the chessboard.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Square {
    /// The square is empty.
    Empty,
    /// The square contains a chess piece.
    Full(Piece),
}


/// Represents the entire chessboard as an 8x8 grid.
#[derive(Clone, Debug)]
pub struct BoardState {
    /// 8x8 array representing the board. Each square can be `Empty` or contain a piece.
    pub board: [[Square; BOARD_SIZE]; BOARD_SIZE],
    /// Keeps track of which player's turn it is (White or Black).
    pub to_move: PieceColour,
}

impl BoardState {
    /// Creates a new chessboard with the default starting position.
    pub fn new() -> Self {
        // Initialize the board with all squares empty.
        let mut board = [[Square::Empty; BOARD_SIZE]; BOARD_SIZE];

        // Set up the standard chess starting position.
        Self::setup_pieces(&mut board);

        BoardState {
            board,
            to_move: PieceColour::White, // White moves first.
        }
    }

    /// Sets up the pieces on the board in the standard starting position.
    fn setup_pieces(board: &mut [[Square; BOARD_SIZE]; BOARD_SIZE]) {
        // Place Black pieces (back rank).
        board[0][0] = Square::Full(Piece {
            kind: PieceKind::Rook,
            colour: PieceColour::Black,
        });
        board[0][1] = Square::Full(Piece {
            kind: PieceKind::Knight,
            colour: PieceColour::Black,
        });
        board[0][2] = Square::Full(Piece {
            kind: PieceKind::Bishop,
            colour: PieceColour::Black,
        });
        board[0][3] = Square::Full(Piece {
            kind: PieceKind::Queen,
            colour: PieceColour::Black,
        });
        board[0][4] = Square::Full(Piece {
            kind: PieceKind::King,
            colour: PieceColour::Black,
        });
        board[0][5] = Square::Full(Piece {
            kind: PieceKind::Bishop,
            colour: PieceColour::Black,
        });
        board[0][6] = Square::Full(Piece {
            kind: PieceKind::Knight,
            colour: PieceColour::Black,
        });
        board[0][7] = Square::Full(Piece {
            kind: PieceKind::Rook,
            colour: PieceColour::Black,
        });

        // Place Black pawns.
        for col in 0..BOARD_SIZE {
            board[1][col] = Square::Full(Piece {
                kind: PieceKind::Pawn,
                colour: PieceColour::Black,
            });
        }

        // Place White pieces (back rank).
        board[7][0] = Square::Full(Piece {
            kind: PieceKind::Rook,
            colour: PieceColour::White,
        });
        board[7][1] = Square::Full(Piece {
            kind: PieceKind::Knight,
            colour: PieceColour::White,
        });
        board[7][2] = Square::Full(Piece {
            kind: PieceKind::Bishop,
            colour: PieceColour::White,
        });
        board[7][3] = Square::Full(Piece {
            kind: PieceKind::Queen,
            colour: PieceColour::White,
        });
        board[7][4] = Square::Full(Piece {
            kind: PieceKind::King,
            colour: PieceColour::White,
        });
        board[7][5] = Square::Full(Piece {
            kind: PieceKind::Bishop,
            colour: PieceColour::White,
        });
        board[7][6] = Square::Full(Piece {
            kind: PieceKind::Knight,
            colour: PieceColour::White,
        });
        board[7][7] = Square::Full(Piece {
            kind: PieceKind::Rook,
            colour: PieceColour::White,
        });

        // Place White pawns.
        for col in 0..BOARD_SIZE {
            board[6][col] = Square::Full(Piece {
                kind: PieceKind::Pawn,
                colour: PieceColour::White,
            });
        }
    }

    /// Prints the board in a human-readable format.
    pub fn print_board(&self) {
        println!("  a b c d e f g h");
        for (i, row) in self.board.iter().enumerate() {
            print!("{} ", 8 - i); // Print row numbers in chess notation.
            for square in row.iter() {
                match square {
                    Square::Empty => print!(". "), // Empty square.
                    Square::Full(piece) => {
                        // Represent pieces with letters (uppercase for White, lowercase for Black).
                        let symbol = match (piece.colour, piece.kind) {
                            (PieceColour::White, PieceKind::Pawn) => 'P',
                            (PieceColour::White, PieceKind::Knight) => 'N',
                            (PieceColour::White, PieceKind::Bishop) => 'B',
                            (PieceColour::White, PieceKind::Rook) => 'R',
                            (PieceColour::White, PieceKind::Queen) => 'Q',
                            (PieceColour::White, PieceKind::King) => 'K',
                            (PieceColour::Black, PieceKind::Pawn) => 'p',
                            (PieceColour::Black, PieceKind::Knight) => 'n',
                            (PieceColour::Black, PieceKind::Bishop) => 'b',
                            (PieceColour::Black, PieceKind::Rook) => 'r',
                            (PieceColour::Black, PieceKind::Queen) => 'q',
                            (PieceColour::Black, PieceKind::King) => 'k',
                        };
                        print!("{} ", symbol);
                    }
                }
            }
            println!("{}", 8 - i); // Print row numbers again on the right.
        }
        println!("  a b c d e f g h");
    }


    /// Returns the piece at the given position, if there is one.
    pub fn get_piece_at(&self, row: usize, col: usize) -> Option<Piece> {
        match self.board[row][col] {
            Square::Full(piece) => Some(piece),
            Square::Empty => None,
        }
    }

    /// Removes the piece at the given position and sets the square to empty.
    pub fn clear_square(&mut self, row: usize, col: usize) {
        self.board[row][col] = Square::Empty;
    }

    /// Switches the turn to the other player.
    pub fn switch_turn(&mut self) {
        self.to_move = match self.to_move {
            PieceColour::White => PieceColour::Black,
            PieceColour::Black => PieceColour::White,
        };
        debug!("Turn changes to {:?}", self.to_move);
    }

    /*
    // Fen handling

    /// Parses a FEN string to initialize the board.
    pub fn from_fen(fen: &str) -> Result<Self, &'static str> {
        // Parse the FEN string into a BoardState.
        // Handle errors for invalid FEN strings.
    }

    /// Converts the board state back to a FEN string.
    pub fn to_fen(&self) -> String {
        // Generate the FEN representation of the current board state.
    }
    */
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_initialization() {
        let board = BoardState::new();
        // Verify that all white pieces are in their correct starting positions.
        assert!(matches!(board.board[7][0], Square::Full(Piece { kind: PieceKind::Rook, colour: PieceColour::White })));
        assert!(matches!(board.board[7][1], Square::Full(Piece { kind: PieceKind::Knight, colour: PieceColour::White })));
        assert!(matches!(board.board[7][2], Square::Full(Piece { kind: PieceKind::Bishop, colour: PieceColour::White })));
        assert!(matches!(board.board[7][3], Square::Full(Piece { kind: PieceKind::Queen, colour: PieceColour::White })));
        assert!(matches!(board.board[7][4], Square::Full(Piece { kind: PieceKind::King, colour: PieceColour::White })));
        assert!(matches!(board.board[7][5], Square::Full(Piece { kind: PieceKind::Bishop, colour: PieceColour::White })));
        assert!(matches!(board.board[7][6], Square::Full(Piece { kind: PieceKind::Knight, colour: PieceColour::White })));
        assert!(matches!(board.board[7][7], Square::Full(Piece { kind: PieceKind::Rook, colour: PieceColour::White })));

        // Verify that all black pieces are in their correct starting positions.
        assert!(matches!(board.board[0][0], Square::Full(Piece { kind: PieceKind::Rook, colour: PieceColour::Black })));
        assert!(matches!(board.board[0][1], Square::Full(Piece { kind: PieceKind::Knight, colour: PieceColour::Black })));
        assert!(matches!(board.board[0][2], Square::Full(Piece { kind: PieceKind::Bishop, colour: PieceColour::Black })));
        assert!(matches!(board.board[0][3], Square::Full(Piece { kind: PieceKind::Queen, colour: PieceColour::Black })));
        assert!(matches!(board.board[0][4], Square::Full(Piece { kind: PieceKind::King, colour: PieceColour::Black })));
        assert!(matches!(board.board[0][5], Square::Full(Piece { kind: PieceKind::Bishop, colour: PieceColour::Black })));
        assert!(matches!(board.board[0][6], Square::Full(Piece { kind: PieceKind::Knight, colour: PieceColour::Black })));
        assert!(matches!(board.board[0][7], Square::Full(Piece { kind: PieceKind::Rook, colour: PieceColour::Black })));

        // Verify that pawns are in their correct starting positions.
        for col in 0..BOARD_SIZE {
            assert!(matches!(board.board[6][col], Square::Full(Piece { kind: PieceKind::Pawn, colour: PieceColour::White })));
            assert!(matches!(board.board[1][col], Square::Full(Piece { kind: PieceKind::Pawn, colour: PieceColour::Black })));
        }

        // Verify that all other squares are empty.
        for row in 2..6 {
            for col in 0..BOARD_SIZE {
                assert!(matches!(board.board[row][col], Square::Empty));
            }
        }
    }

    #[test]
    fn test_print_board() {
        let board = BoardState::new();
        board.print_board(); // Verify visually that the output matches the expected initial setup.
    }

    #[test]
    fn test_to_move_initial_state() {
        let board = BoardState::new();
        assert_eq!(board.to_move, PieceColour::White); // Verify white moves first.
    }

    #[test]
    fn test_place_piece() {
        let mut board = BoardState::new();

        // Place a white bishop on d4.
        board.board[3][3] = Square::Full(Piece {
            kind: PieceKind::Bishop,
            colour: PieceColour::White,
        });
        assert!(matches!(board.board[3][3], Square::Full(Piece { kind: PieceKind::Bishop, colour: PieceColour::White })));

        // Place a black knight on e5.
        board.board[4][4] = Square::Full(Piece {
            kind: PieceKind::Knight,
            colour: PieceColour::Black,
        });
        assert!(matches!(board.board[4][4], Square::Full(Piece { kind: PieceKind::Knight, colour: PieceColour::Black })));
    }

    #[test]
    fn test_clear_piece() {
        let mut board = BoardState::new();

        // Clear a square with a piece.
        board.board[6][4] = Square::Empty; // Clear the e2 pawn.
        assert!(matches!(board.board[6][4], Square::Empty));
    }

    

    #[test]
    fn test_modify_to_move() {
        let mut board = BoardState::new();

        // Change the turn to Black.
        board.to_move = PieceColour::Black;
        assert_eq!(board.to_move, PieceColour::Black);

        // Change back to White.
        board.to_move = PieceColour::White;
        assert_eq!(board.to_move, PieceColour::White);
    }

    #[test]
    fn test_custom_board_setup() {
        let mut board = BoardState::new();

        // Create a custom board state: two kings only.
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                board.board[row][col] = Square::Empty;
            }
        }

        board.board[7][4] = Square::Full(Piece {
            kind: PieceKind::King,
            colour: PieceColour::White,
        });
        board.board[0][4] = Square::Full(Piece {
            kind: PieceKind::King,
            colour: PieceColour::Black,
        });

        // Validate custom setup.
        assert!(matches!(board.board[7][4], Square::Full(Piece { kind: PieceKind::King, colour: PieceColour::White })));
        assert!(matches!(board.board[0][4], Square::Full(Piece { kind: PieceKind::King, colour: PieceColour::Black })));
        for row in 1..7 {
            for col in 0..BOARD_SIZE {
                assert!(matches!(board.board[row][col], Square::Empty));
            }
        }
    }
}
