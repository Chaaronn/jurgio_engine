use board::BoardState;

mod board;
mod pieces;
mod game_logic;
mod moves;

fn main() {

    let board = BoardState::new();

    board.print_board();
}
