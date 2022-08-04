use chess::chess::board::Board;
use chess::chess::crd::Crd;
    

#[test]
fn test_pawn_transformation() {
    let board_usize: [[usize; 8]; 8] = 
        [
            [4, 6, 8, 0, 12,8, 6, 4],
            [2, 2, 2, 1, 2, 2, 2, 2],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 9, 0, 0, 0, 0],
            [1, 0, 1, 1, 1, 1, 1, 1],
            [3, 1, 0, 0, 11, 7, 5, 3],
        ];

    let mut board: Board = Board::create();
    board.from(board_usize);

    board.move_piece(&Crd::create(1,3).unwrap(), &Crd::create(0,3).unwrap());
    //let moves = moves.get(&Crd::create(4, 1).unwrap()).unwrap();
    let piece = board.get_player_piece(&Crd::create(0,3).unwrap(), 1).unwrap();


    assert!(piece.is_queen());

}