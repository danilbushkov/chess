
mod piece;
mod board;
mod algorithm;
mod context;
pub mod state;
pub mod code;
pub mod crd;



use crate::chess::code::Code;
use crate::chess::crd::Crd;
use crate::chess::context::Context;



pub struct Chess{
    context: Context,

    
}

impl Chess {

    pub fn create() -> Self {
        Self{
            context: Context::create(),
        }
    }

    pub fn handler(&mut self, crd: Option<Crd>) -> Code {
        let state = self.context.get_state();
        match state {
            Some(s) => s.handler(&mut self.context, crd),
            None => Code::NoneState,
        }
    }

    pub fn get_board_i8(&self) -> [[i8; 8]; 8] {
        self.context.get_board_i8()
    }

    pub fn get_color_board(&self) -> [[(i8, i8); 8]; 8] {
        self.context.get_color_board()
    }

    pub fn init(&mut self){
        self.context.init();
    }

    

}