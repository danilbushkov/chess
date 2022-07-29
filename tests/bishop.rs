use chess::chess::piece::Piece;
use chess::chess::board::Board;
use std::collections::HashSet;
use chess::chess::crd::Crd;
    

#[test]
fn check_move_bishop() {
    let board_i8: [[i8; 8]; 8] = 
        [
            [4, 6, 8,10, 12,8, 6, 4],
            [2, 2, 2, 2, 2, 2, 2, 2],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0],
            [1, 1, 1, 0, 1, 1, 1, 1],
            [3, 5, 7, 9, 11,7, 5, 3],
        ];

    let mut board: Board = Board::create();
    board.from(board_i8);
    let crd = Crd::create(7, 2);
    let piece = board.get_piece(&crd).unwrap();
    

    assert_eq!(piece.moves(&crd.unwrap(), &board), HashSet::from([(6, 3)]));
}