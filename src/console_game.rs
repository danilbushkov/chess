
use std::io;

use crate::chess::Chess;


pub struct ConsoleGame{
    chess: Chess,
}



impl ConsoleGame {

    pub fn create(chess: Chess) -> Self{
        Self{
            chess,
        }
    }


    pub fn run(&self){
        self.menu();
    }

    pub fn menu(&self){
        println!("Menu:");
        println!("1 - play");
        println!("2 - exit");
        self.read_code_menu();
    }

    pub fn read_code_menu(&self){

        
        let mut input = String::new();

        print!("Enter the item: ");

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("Yes");
            }
            Err(error) => println!("Error: {error}"),
        }
    }
}

