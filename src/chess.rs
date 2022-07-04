//  white/black
// 0 - empty cell
// 1/2 - pawn
// 3/4 - rook
// 5/6 - horse
// 7/8 - bishop
// 9/10 - queen
// 11/12 - king


pub struct Chess{
    board: [[i8; 8]; 8],

}

impl Chess {

    pub fn new() -> Self{
        Self {
            board: [[0; 8]; 8],
        }
    }

    pub fn init_board(&mut self) {
        self.board = [
            [4, 6, 8,10, 12,8, 6, 4],
            [2, 2, 2, 2, 2, 2, 2, 2],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [3, 5, 7, 9, 11,7, 5, 3],
        ];
    }

    pub fn print_board(&self) {
        for arr in self.board {
            for el in arr {
                print!("{el:^4}");
            }
            println!();
        }
    }



    pub fn run(&self) {
        println!("run!");
    }

}