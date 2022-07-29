
use std::collections::HashSet;
use crate::chess::board::Board;
use crate::chess::crd::Crd;
use crate::chess::state::State;
use crate::chess::piece::Piece;

pub struct Context {
    player: usize,
    board: Board,
    state: Option<State>,
    moves: HashSet<(usize, usize)>,
    piece_crd: Option<Crd>,
    //move_crd: Option<Crd>,
}


impl Context {
    pub fn create() -> Self {

        Self {
            player: 1,
            moves: HashSet::new(),
            board: Board::create(),
            state: Some(State::SelectPieceState),
            piece_crd: Crd::default(),
            
        }
    }

    pub fn get_state(&mut self) -> Option<State> {
        self.state.take()
    }

    pub fn change_state(&mut self, state: State){
        state.action_on_change(self);
        self.state = Some(state);
    }


  //------------------------------------------  

    pub fn get_possible_moves(&mut self, crd: &Crd) -> HashSet<(usize, usize)> {
        
        match self.get_piece_by_crd(&crd) {
            Some(piece) => return piece.moves(&crd ,&self.board),
            None => (),
        }
        HashSet::new()
    }

    pub fn check_possible_move(&self, crd: &Crd) -> bool {
        
        self.moves.contains(&crd.get_tuple())
            
    }

    pub fn en_passant(&mut self, crd: &Crd) -> bool {
        if let Some(player_crd) = &self.piece_crd {
            // if !self.board.is_player_piece(player_crd, self.player) {
            //     return false;
            // }
            if let Some(player) = self.board.get_player_piece(player_crd, self.player) {

                if !player.is_pawn() {
                    return false;
                }
                let direction = crd.y() - player_crd.y();
                if direction.abs() != 1 {
                    return false;
                }

                if let Some(enemy_crd) = Crd::create(player_crd.x(), player_crd.y() + direction) {
                    if let Some(enemy) = self.board.get_enemy_piece(&enemy_crd, self.player) {
                        if !enemy.is_pawn() {
                            return false;
                        }
                        let m = self.board.move_piece(player_crd, crd);
                        let r = self.board.remove_piece(&enemy_crd);
                        return m && r;
                   }
                }
            }

        }


        // let player = self.get_piece().unwrap();
        // if !player.is_pawn() {
        //     return false;
        // }
        
        //let player_crd = self.piece_crd.as_ref().unwrap();
        //let target_crd = crd.as_ref().unwrap();
        //let direction = target_crd.y() - player_crd.y();
        // if direction.abs() != 1 {
        //     return false;
        // }
        // let enemy_crd = Crd::create(player_crd.x(), player_crd.y() + direction);
        // if let Some(enemy) = self.get_piece_by_crd(&enemy_crd) {
        //     if enemy.is_pawn() {
        //         if let Some(enemy_crd) = enemy_crd {
        //             self.board.move_piece(player_crd, target_crd);
        //             self.board.remove_piece(&enemy_crd);
        //             return true;
        //         }
        //     }
        // }
        
        false
    }
    
    pub fn move_piece(&mut self, crd: &Crd) -> bool {
        if let Some(player_crd) = &self.piece_crd {
            if !self.board.is_player_piece(player_crd, self.player) {
                return false;
            }
            return self.board.move_piece(player_crd, crd);
        }
        
        false
    }

    pub fn capture(&mut self, crd: &Crd) -> bool {
        if let Some(player_crd) = &self.piece_crd {
            if !self.board.is_player_piece(player_crd, self.player) {
                return false;
            }
            if self.board.is_enemy_piece(crd, self.player) {
                return self.board.capture(player_crd, crd);
            }
        }

        false
    }

  //------------------------------------------
    pub fn change_player(&mut self) {
        self.player = match self.player {
            1 => 2,
            2 => 1,
            _ => 1,
        };
    }
    // .0 - cell, .1 - color cell
    // color:
    // 0 - normal
    // 1 - move
    pub fn get_color_board(&self) -> [[(usize, usize); 8]; 8] {
        let board = self.get_board_usize();
        let mut color_board: [[(usize, usize); 8]; 8] = [[(0, 0); 8]; 8];
        for (i, arr) in board.iter().enumerate() {
            for (j, item) in arr.iter().enumerate() {
                color_board[i][j].0 = *item;
            }   
        }
        for item in &self.moves {
            color_board[item.0][item.1].1 = 1; 
        }


        color_board
    }

    pub fn get_board_usize(&self) -> [[usize; 8]; 8] {
        self.board.get_board()
    }

    pub fn init(&mut self){
        self.board.init();
    }

    // pub fn get_moves(&mut self, moves: Vec<Crd>) {
    //     self.moves = moves;
    // }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_piece_crd(&self) -> &Option<Crd> {
        &self.piece_crd
    }

    pub fn set_piece_crd(&mut self, crd: Crd) {
        self.piece_crd = Some(crd);
    }

    // pub fn get_move_crd(&self) -> &Crd {
    //     &self.move_crd
    // }

    // pub fn set_move_crd(&mut self, crd: Option<Crd>) {
    //     self.move_crd = crd;
    // }

    pub fn get_piece(&self) -> Option<&Box<Piece>> {
        match &self.get_piece_crd() {
            Some(c) => self.board.get_piece(c),
            None => None,
        }
    }

    // pub fn get_piece_mut(&mut self) -> Option<&mut Box<Piece>> {
    //     self.board.get_piece_mut(&self.get_piece_crd())
    // }

    pub fn get_piece_by_crd(&self, crd: &Crd) -> Option<&Box<Piece>> {
        self.board.get_piece(crd)
    }

    pub fn check_moves(&self) -> bool {
        !self.moves.is_empty()
    }

    pub fn clear_moves(&mut self) {
        self.moves.clear();
    }

    pub fn set_moves(&mut self, moves: HashSet<(usize, usize)>) {
        self.moves = moves;
    }

    pub fn is_player_piece(&self, crd: &Crd) -> bool {
        self.board.is_player_piece(crd, self.player)
    }
}