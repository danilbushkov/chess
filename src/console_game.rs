
use std::io;

use crate::chess::Chess;
use crate::chess::code::Code;


pub struct ConsoleGame {
    chess: Chess,
}



impl ConsoleGame {

    pub fn create(chess: Chess) -> Self {
        Self{
            chess,
        }
    }


    pub fn run(&self){
        self.clear_screen();
        let mut code: i8 = 0;

        while code != 2 {
            code = self.menu();

            self.code_selection(code);

        }
    }

    fn code_selection(&self, code: i8){
        match code {
            1 => self.game(),
            _ => (),
        }
    }

    fn game(&self) {
        let mut code: Code = Code::None;
        while match code {
            Code::Exit => false,
            _ => true,
        } { code = self.game_move(); }
    
    }

    fn game_move(&self) -> Code {
        //print board

        println!("Enter coordinates: ");
        let args = self.get_vec_crd();

        self.clear_screen();
        if args.len() == 1 {
            if args[0] == "exit" || args[0] == "Exit" {
                return Code::Exit;
            }
        } else if args.len() == 2 {
            

        } else {
            self.println_error("There are too many arguments"); 
        }

        Code::None
    }

    fn get_vec_crd(&self) -> Vec<String> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {

                input.split_whitespace()
                    .map(|x| x.to_string())
                    .collect()
                
            }
            Err(error) => {
                
                println!("Error: {error}");
                vec![]
                
            }
        }
    }



    //0 - error
    fn menu(&self) -> i8 {
        
        println!("Menu:");
        println!("1 - play");
        println!("2 - exit");
        println!("Enter the item: ");

        self.select_menu()
    }




    fn select_menu(&self) -> i8 {
        
        let mut input = String::new();
        self.clear_screen();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {

                let string = input.trim();
                match string.parse::<i8>() {
                    Ok(n) => {
                        n 
                    } 
                    Err(_) => {
                        self.println_error("There is no such menu item");
                        0
                    }
                }
            }
            Err(error) => {
                
                println!("Error: {error}");
                0
            }
        }
    }

    fn println_error(&self ,s: &str){
        println!("\x1B[31;1m{}{}\x1B[0m", "Error: ", s);
    }
    


    fn clear_screen(&self){
        print!("\x1B[2J\x1B[1;1H");
    }
}

