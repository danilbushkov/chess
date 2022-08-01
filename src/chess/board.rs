use std::collections::HashMap;
use std::collections::HashSet;

use crate::chess::piece::Piece;
use crate::chess::crd::Crd;
use crate::chess::piece::trajectory::Piece as Trajectory;

pub struct Board {
    board: Vec<Vec<Option<Box<Piece>>>>,
    //0 - white, 1 - black
    pieces: [HashSet<Crd>; 2],
}


impl Board {
    
    pub fn create() -> Self {
        let mut board: Vec<Vec<Option<Box<Piece>>>> = Vec::with_capacity(8);
        
        for i in 0..8 {
            board.push(Vec::with_capacity(8));
            for _ in 0..8 {
                board[i].push(None);
            }
        }
        
        let pieces = [HashSet::with_capacity(16), HashSet::with_capacity(16)];

        Self {
            board,
            pieces,
        }
    }

    pub fn get_board(&self) -> [[usize; 8]; 8] {
        let mut board: [[usize; 8]; 8] = [[0; 8]; 8];

        for (i, arr) in self.board.iter().enumerate() {
            for (j, item) in arr.iter().enumerate() {
                board[i][j] = match item {
                    Some(i) => i.get_code(),
                    None => 0,
                }
            }
        }

        board
    }

    pub fn init(&mut self){
        self.from(self.start_board());
    }


    pub fn from(&mut self, board_isize: [[usize; 8]; 8]) {
        self.pieces[0].clear();
        self.pieces[1].clear();
        for (i, arr) in board_isize.iter().enumerate() {
            for (j, item) in arr.iter().enumerate() {
                self.board[i][j] = match *item {
                    0 => None,
                    other => {
                        let piece = Some(
                            Box::new(
                                Piece::create(other)
                            )
                        );
                        if let Some(ref p) = piece {
                            if let Some(crd) = Crd::create_u((i, j)) {
                                self.pieces[(p.get_player()/2) as usize].insert(crd);
                            }
                            
                        }


                        piece
                    },
                }; 
                
            } 
        }
    }

    fn start_board(&self) -> [[usize; 8]; 8] {
        [
            [4, 6, 8,10, 12,8, 6, 4],
            [2, 2, 2, 2, 2, 2, 2, 2],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [3, 5, 7, 9, 11,7, 5, 3],
        ]
    }

    

    pub fn get_piece(&self, crd: &Crd) -> Option<&Box<Piece>> {
        self.get_ref(crd.get_tuple())
    }

   


    pub fn move_piece(&mut self, location: &Crd, target: &Crd) -> bool {
        
        let mut tmp = self.take(location.get_tuple());

        if let Some(t) = &mut tmp {
            if t.get_player() > 0 {
                
                t.change_first_move();
                t.change_two_calls(location, target);

                self.pieces[(t.get_player()/2)].remove(location);
                self.pieces[(t.get_player()/2)].insert(target.clone());
                self.set(target.get_tuple(), tmp);
                return true;
            }
            
        }
        

        false
    }   

    pub fn remove_piece(&mut self, target: &Crd) -> bool {
        let piece = self.take(target.get_tuple());

        if let Some(ref t) = piece {
            if t.get_player() > 0 {
                self.pieces[(t.get_player()/2) ].remove(target);
                return true;
            } 
        }
        false
    }

    pub fn capture(&mut self, location: &Crd, target: &Crd) -> bool {

        if let Some(ref m) = self.get_ref(location.get_tuple()) {
            if let Some(ref r) = self.get_ref(target.get_tuple()) {
                let player = m.get_player();
                let enemy = r.get_player();
                if player == 0 || enemy == 0 {
                    return false;
                }

                
                let mut tmp = self.take(location.get_tuple());
                if let Some(p) = &mut tmp {
                    p.change_first_move();
                    self.set(target.get_tuple(), tmp);
                
                    self.pieces[enemy/2].remove(target);
                    self.pieces[player/2].remove(location);
                    self.pieces[player/2].insert(target.clone());
                    return true;
                }
            }  
        }
        false
    }
    

    pub fn is_piece(&self, crd: &Crd) -> bool {
        match self.get_piece(crd) {
            Some(p) => match **p {
                Piece::None => false,
                _ => true,
            },
            None => false,
        }
    }
    pub fn is_piece_by_crd(&self, crd: &Crd) -> bool {
        match self.get_piece(crd) {
            Some(p) => match **p {
                Piece::None => false,
                _ => true,
            },
            None => false,
        }
    }

    pub fn _check_pin(&mut self, player_crd: &Crd, threats: &HashMap<Crd, HashSet<Crd>>) -> bool {
        for (_, way) in threats {
            for crd in way {
                if player_crd == crd {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_match(&self, moves: &mut HashSet<Crd>, threats: &Vec<HashSet<Crd>>) {
        let mut tmp_moves: HashSet<Crd> = HashSet::new();
        for way in threats {
            for crd in way {
                if moves.contains(&crd) {
                    tmp_moves.insert(crd.clone());
                }
            }
        }
        *moves = tmp_moves;
    }
    pub fn get_player_moves(&self, crd: &Crd, current_player: usize) -> HashSet<Crd> {
        if let Some(piece) = self.get_player_piece(crd, current_player) {
            let mut moves = piece.moves(crd, self);
            if !moves.is_empty() && !piece.is_king() {
                
                let threats = self.threatening_player_king(crd ,current_player);
                if threats.len() > 1 {
                    moves = HashSet::new();
                } else if threats.len() == 1 {
                    self.get_match(&mut moves, &threats);
                }

            }
            return moves;
        }
        HashSet::new()

    }


    pub fn threatening_player_king(&self, skip: &Crd, current_player: usize) -> Vec<HashSet<Crd>> {
        let mut pieces = Vec::new();
        for enemy_crd in self.get_enemy_pieces(current_player) {
            if let Some(piece) = self.get_enemy_piece(enemy_crd, current_player) {
                for player_crd in piece.moves(enemy_crd, self) {
                    if let Some(piece) = self.get_player_piece(&player_crd, current_player) {
                        if piece.is_king() {
                            
                            pieces.push(self.way_to_king(&player_crd, enemy_crd, skip, current_player));

                            
                        }
                    }
                }
            }
        }
        pieces
    }


    pub fn way_to_king(&self, king_crd: &Crd, enemy_crd: &Crd, skip: &Crd, current_player: usize) -> HashSet<Crd> {
        let mut trajectory: HashSet<Crd> = HashSet::new();
        if let Some(enemy) = self.get_enemy_piece(enemy_crd, current_player) {
            if enemy.is_queen_or_bishop_or_rook() {
                let a = (king_crd.x() - enemy_crd.x()).signum();
                let b = (king_crd.y() - enemy_crd.y()).signum();
                let d = (a, b);
                Trajectory::get_trajectory(&mut trajectory, enemy_crd, &self, &d, current_player, Some(skip));
                trajectory.remove(&king_crd);
            }
            trajectory.insert(enemy_crd.clone());
        }

        trajectory
    }


    pub fn get_possible_moves(&mut self, current_player: usize) -> HashMap<Crd, HashSet<Crd>> {
        let mut players_moves: HashMap<Crd, HashSet<Crd>> = HashMap::new();
        let pieces = self.get_player_pieces(current_player);

        for crd in pieces {
            if self.is_player_piece(&crd, current_player) {
                
                let moves = self.get_player_moves(&crd, current_player);
                if !moves.is_empty() {
                    players_moves.insert(crd.clone(), moves);
                }
            }
        }
        players_moves
    }


    pub fn get_enemy_attacks(&self, current_player: usize, skip: &Crd) -> HashSet<Crd> {
        let mut cells: HashSet<Crd> = HashSet::new();
        for enemy_crd in self.get_enemy_pieces(current_player) {
            if let Some(enemy) = self.get_enemy_piece(enemy_crd, current_player) {
                cells.extend(enemy.attacks(enemy_crd, self, skip));
            }
        }

        cells
    }

   





    pub fn is_enemy_piece(&self, crd: &Crd, current_player: usize) -> bool {
        match self.get_piece(crd) {
            Some(p) => match **p {
                Piece::None => false,
                ref other => other.get_player() == [1, 2][(current_player % 2)],
            },
            None => false,
        }

    }

    pub fn get_enemy_piece(&self, crd: &Crd, current_player: usize) -> Option<&Box<Piece>> {
        match self.get_ref(crd.get_tuple()) {
            Some(ref p) => {
                if p.get_player() == [1, 2][(current_player % 2)] {
                    return Some(p);
                }
                None
            },
            None => None, 
        }   
    }

    pub fn get_player_piece(&self, crd: &Crd, current_player: usize) -> Option<&Box<Piece>> {
        match self.get_ref(crd.get_tuple()) {
            Some(ref p) => {
                if p.get_player() == current_player {
                    return Some(p);
                }
                None
            },
            None => None, 
        }   
    }

    pub fn is_player_piece(&self, crd: &Crd, current_player: usize) -> bool {
        match self.get_piece(crd) {
            Some(p) => match **p {
                Piece::None => false,
                ref other => other.get_player() == current_player,
            },
            None => false,
        }
        
    }

    pub fn get_ref(&self, (x, y): (usize, usize)) -> Option<&Box<Piece>> {
        self.board[x][y].as_ref()
    }

    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut Box<Piece>> {
        self.board[x][y].as_mut()
    }
    pub fn take(&mut self, (x, y): (usize, usize)) -> Option<Box<Piece>> {
        self.board[x][y].take()
    }

    pub fn set(&mut self, (x, y): (usize, usize), value: Option<Box<Piece>>) {
        self.board[x][y] = value;
    }
    
    pub fn get_pieces(&self) -> &[HashSet<Crd>; 2] {
        &self.pieces
    }
    pub fn get_player_pieces(&self, player: usize) -> &HashSet<Crd> {
        &self.pieces[player/2]
    }
    pub fn get_enemy_pieces(&self, player: usize) -> &HashSet<Crd> {
        &self.pieces[player%2]
    }
}