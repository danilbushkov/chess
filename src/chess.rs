
pub mod piece;
pub mod board;
mod context;
pub mod state;
pub mod code;
pub mod crd;



use crate::chess::code::Code;
use crate::chess::crd::Crd;
use crate::chess::context::Context;



pub struct Chess {
    context: Context,

}

impl Chess {

    pub fn create() -> Self {

        let mut context = Context::create();
        context.init();
        

        Self{
            context,
        }
    }

    pub fn handler(&mut self, crd: Option<Crd>) -> Code {
        match crd {
            Some(c) => {
                let state = self.context.get_state();
                match state {
                    Some(s) => s.handler(&mut self.context, c),
                    None => Code::NoneState,
                }
            } 
            None => Code::IncorrectCrd,
        }
    }

    pub fn get_board_usize(&self) -> [[usize; 8]; 8] {
        self.context.get_board_usize()
    }

    pub fn get_color_board(&self) -> [[(usize, usize); 8]; 8] {
        self.context.get_color_board()
    }

    // pub fn init(&mut self){
    //     //self.context.init();
    // }

    

}