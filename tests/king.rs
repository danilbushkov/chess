use chess::chess::board::Board;
use std::collections::HashSet;
use chess::chess::crd::Crd;
    

#[test]
fn king1() {
    let board_usize: [[usize; 8]; 8] = 
        [
            [4, 6, 8, 0, 12,8, 6, 4],
            [2, 2, 2, 2, 2, 2, 2, 2],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 9, 0, 0, 0, 0, 0],
            [0, 11, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 9, 0, 0, 0, 0],
            [1, 0, 1, 0, 1, 1, 1, 1],
            [3, 1, 0, 0, 0, 7, 5, 3],
        ];

    let mut board: Board = Board::create();
    board.from(board_usize);

    let moves = board.get_possible_moves(1);
    let moves = moves.get(&Crd::create(4, 1).unwrap()).unwrap();

    let result = HashSet::from([   
        Crd::create(3, 0).unwrap(),
        Crd::create(4, 0).unwrap(),
        Crd::create(5, 0).unwrap(),
        Crd::create(3, 1).unwrap(),
        Crd::create(4, 2).unwrap(),
        Crd::create(5, 2).unwrap(),
        ]);


    assert_eq!(*moves, result);
}
#[test]
fn king2() {
    let board_usize: [[usize; 8]; 8] = 
        [
            [4, 6, 8, 0, 12,8, 6, 4],
            [2, 2, 2, 2, 2, 2, 2, 2],
            [0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 9, 0, 0, 0, 0],
            [1, 0, 0, 0, 1, 1, 1, 1],
            [3, 0, 0, 0, 11, 0, 0, 3],
        ];

    let mut board: Board = Board::create();
    board.from(board_usize);

    let moves = board.get_possible_moves(1);
    let moves = moves.get(&Crd::create(7, 4).unwrap()).unwrap();

    let result = HashSet::from([   
        Crd::create(7, 6).unwrap(),
        Crd::create(7, 5).unwrap(),
        Crd::create(7, 3).unwrap(),
        Crd::create(6, 3).unwrap(),
        Crd::create(7, 1).unwrap(),
        ]);


    assert_eq!(*moves, result);
}