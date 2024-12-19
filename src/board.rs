
pub const BOARD_SIZE: usize = 8;

// Possible piece colours
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceColour {
    White,
    Black,
}

impl PieceColour {
    /// Returns the opposite colour. (e.g., White -> Black, Black -> White)
    pub fn opposite(self) -> Self {
        match self {
            PieceColour::White => PieceColour::Black,
            PieceColour::Black => PieceColour::White,
        }
    }
}

/// Represents the different kinds of chess pieces (e.g., Pawn, Knight).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// Represents a chess piece with its kind and colour.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub colour: PieceColour,
}

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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_kind_variants() {
        let pawn = PieceKind::Pawn;
        let knight = PieceKind::Knight;
        let bishop = PieceKind::Bishop;
        let rook = PieceKind::Rook;
        let queen = PieceKind::Queen;
        let king = PieceKind::King;

        assert_eq!(pawn as usize, 0); // or any relevant test
        assert_eq!(king as usize, 5); // or any relevant test
    }
}