use chess::chess::piece::Piece;
use chess::chess::board::Board;
use std::collections::HashSet;
use chess::chess::crd::Crd;
    

#[test]
fn check_move_bishop() {
    let board_usize: [[usize; 8]; 8] = 
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
    board.from(board_usize);
    let crd = Crd::create(7, 2);
    let piece = board.get_piece(&crd.as_ref().unwrap()).unwrap();


    assert_eq!(piece.moves(&crd.as_ref().unwrap(), &board), HashSet::from([(6, 3)]));
}