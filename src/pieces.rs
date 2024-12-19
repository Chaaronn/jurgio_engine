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