use chess::chess::board::Board;
use std::collections::HashSet;
use std::collections::HashMap;
use chess::chess::crd::Crd;
    

#[test]
fn check1() {
    let board_usize: [[usize; 8]; 8] = 
        [
            [4, 6, 8, 0, 12,8, 6, 4],
            [2, 2, 2, 2, 2, 2, 2, 2],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 9, 0, 0, 0, 0, 0],
            [10, 0, 0, 0, 11, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 0],
            [1, 0, 1, 0, 1, 1, 1, 1],
            [3, 5, 7, 9, 0,7, 5, 3],
        ];

    let mut board: Board = Board::create();
    board.from(board_usize);
    

    let moves = board.get_possible_moves(1);


    
    let result1 = vec![HashSet::from([
        Crd::create(4, 0).unwrap(),
        Crd::create(4, 1).unwrap(),
        Crd::create(4, 2).unwrap(),
        Crd::create(4, 3).unwrap(),
        ])];

    

    assert_eq!(board.threatening_player_king(&Crd::create(3, 2).unwrap(),1), result1);

    let result = HashMap::from([   
        ( Crd::create(3, 2).unwrap(), 
                        HashSet::from([Crd::create(4, 2).unwrap(),
                                       Crd::create(4, 1).unwrap(),
                                       Crd::create(4, 3).unwrap(),
                                       ])),
        ( Crd::create(7, 3).unwrap(), 
                        HashSet::from([Crd::create(4, 3).unwrap()])),
        ( Crd::create(5, 1).unwrap(), 
                        HashSet::from([Crd::create(4, 1).unwrap(),
                                       Crd::create(4, 0).unwrap()])),
        ( Crd::create(6, 2).unwrap(), 
                        HashSet::from([Crd::create(4, 2).unwrap()])),
        ( Crd::create(4, 4).unwrap(), 
                        HashSet::from([Crd::create(3, 3).unwrap(),
                                       Crd::create(3, 4).unwrap(),
                                       Crd::create(3, 5).unwrap(),
                                       Crd::create(5, 3).unwrap(),
                                       Crd::create(5, 4).unwrap(),
                                       Crd::create(5, 5).unwrap(),
                        
                        
                        ]))
        
    ]);


    assert_eq!(moves, result);
}