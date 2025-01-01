use board::BoardState;
use tracing::{info, span, Level};
use tracing_subscriber;

mod board;
mod pieces;
mod game_logic;
mod moves;
mod zorbist;

fn main() {

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG) // Set log level
        .init();

    //let board = BoardState::new();

    //board.print_board();
}
