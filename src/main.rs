use picross::*;

mod picross;

fn main() {
    let board = Board::new("mouse");
    board.show_all_keys();
    board.show_grid();
}
