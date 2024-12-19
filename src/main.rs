use board::BoardState;

mod board;

fn main() {

    let board = BoardState::new();

    board.print_board();
}
