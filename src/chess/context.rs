
use std::collections::HashMap;
use std::collections::HashSet;
use crate::chess::board::Board;
use crate::chess::crd::Crd;
use crate::chess::state::State;
use crate::chess::piece::Piece;

pub struct Context {
    player: usize,
    board: Board,
    state: Option<State>,
    moves: HashMap<Crd, HashSet<Crd>>,
    player_crd: Option<Crd>,
}


impl Context {
    pub fn create() -> Self {

        Self {
            player: 1,
            moves: HashMap::new(),
            board: Board::create(),
            state: Some(State::SelectPieceState),
            player_crd: Crd::default(),
            
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

    pub fn get_possible_moves(&self, crd: &Crd) -> Option<&HashSet<Crd>> {
        
        
        self.moves.get(crd)
        
    }

    pub fn get_player_moves(&mut self, crd: &Crd) -> HashSet<Crd> {
        self.board.get_player_moves(crd, self.player)
    }

    pub fn change_possible_moves(&mut self) {
        self.moves = self.board.get_possible_moves(self.player)
        
    }
    pub fn check_possible_moves(&self, crd: &Crd) -> bool {
        
        self.moves.contains_key(crd) 
 
    }

    pub fn check_possible_move(&self, crd: &Crd) -> bool {
        match self.player_crd {
            Some(ref player_crd) => {
                if let Some(set) = self.moves.get(player_crd) {
                    return set.contains(&crd);
                }
                false
            },
            None => false,
        }
        
            
    }


    pub fn castling(&mut self, crd: &Crd) -> bool {
        if let Some(player_crd) = &self.player_crd {
            if let Some(player) = self.board.get_player_piece(player_crd, self.player) {
                if player.is_king() {
                    if (crd.y() - player_crd.y()).abs() > 1 {
                        let d = (crd.y() - player_crd.y()).signum();
                        let rook_crd = {
                            if d > 0 {
                                Crd::create(crd.x(), 7)
                            } else {
                                Crd::create(crd.x(), 0)
                            }   
                        };
                        if let Some(rc) = rook_crd {
                            if let Some(t) = Crd::create(crd.x(), crd.y()-d) {
                                if self.board.move_piece(player_crd, crd) {
                                    return self.board.move_piece(&rc, &t);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        false
    }




    pub fn en_passant(&mut self, crd: &Crd) -> bool {
        if let Some(player_crd) = &self.player_crd {
            
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
                        if self.board.move_piece(player_crd, crd) {
                            return self.board.remove_piece(&enemy_crd);
                        }
                        
                   }
                }
            }

        }


        
        false
    }
    
    pub fn move_piece(&mut self, crd: &Crd) -> bool {
        if let Some(player_crd) = &self.player_crd {
            if !self.board.is_player_piece(player_crd, self.player) {
                return false;
            }
            return self.board.move_piece(player_crd, crd);
        }
        
        false
    }

    pub fn capture(&mut self, crd: &Crd) -> bool {
        if let Some(player_crd) = &self.player_crd {
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

        if let Some(ref crd) = self.player_crd {
            if let Some(set) = self.moves.get(crd) {
                for item in set {
                    let (a, b) = item.get_tuple();
                    color_board[a][b].1 = 1; 
                }
            }
        }
        
        


        color_board
    }

    pub fn get_board_usize(&self) -> [[usize; 8]; 8] {
        self.board.get_board()
    }

    pub fn init(&mut self){
        self.board.init();
        self.change_possible_moves();
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_player_crd(&self) -> &Option<Crd> {
        &self.player_crd
    }

    pub fn set_player_crd(&mut self, crd: Crd) {
        self.player_crd = Some(crd);
    }


    pub fn get_piece(&self) -> Option<&Box<Piece>> {
        match &self.get_player_crd() {
            Some(c) => self.board.get_piece(c),
            None => None,
        }
    }

    

    pub fn get_piece_by_crd(&self, crd: &Crd) -> Option<&Box<Piece>> {
        self.board.get_piece(crd)
    }

    pub fn check_moves(&self) -> bool {
        !self.moves.is_empty()
    }

    pub fn clear_moves(&mut self) {
        self.moves.clear();
    }


    pub fn is_player_piece(&self, crd: &Crd) -> bool {
        self.board.is_player_piece(crd, self.player)
    }

    pub fn get_player(&self) -> usize {
        self.player
    }


    pub fn get_enemy_piece(&self, crd: &Crd) -> Option<&Box<Piece>> {
        self.board.get_enemy_piece(crd, self.player)
    }

    pub fn get_player_piece(&self, crd: &Crd) -> Option<&Box<Piece>> {
        self.board.get_player_piece(crd, self.player)
    }
}