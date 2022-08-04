
use std::io;

use chess::chess::Chess;
use chess::chess::code::Code;
use chess::chess::crd::Crd;
use rand::Rng;


pub struct ConsoleGame {
    chess: Chess,
}



impl ConsoleGame {

    pub fn create(chess: Chess) -> Self {
        Self{
            chess,
        }
    }


    pub fn run(&mut self){
        self.clear_screen();
        let mut code: usize = 0;

        while code != 3 {
            code = self.menu();

            self.code_selection(code);

        }
    }

    

    fn code_selection(&mut self, code: usize){
        match code {
            1 => self.game(),
            2 => self.bot(),
            _ => (),
        }
    }

    fn game(&mut self) {
        //self.chess.init();
        let mut code: Code = Code::None;
        while match code {
            Code::Exit => false,
            _ => true,
        } { 
            code = self.game_move();
            println!("{}", match code {
                Code::IncorrectCrd => "incorrect crd",
                Code::NonePiece => "none piece",
                Code::NoneState => "none state",
                Code::NoneMoves => "none moves",
                Code::ReselectPiece => "ReselectPiece",
                Code::Exit => "exit",
                Code::None => "none",
                Code::ChangePlayer => "Change player",
            }); 
        
        }
    }

    pub fn bot(&mut self){
        let mut code: Code = Code::None;
        let mut player = 1;
        while match code {
            Code::Exit => false,
            _ => true,
        } { 
            
            if player == 1 {
                code = self.game_move();
            } else {
                let x = rand::thread_rng().gen_range(0..=7);
                let y = rand::thread_rng().gen_range(0..=7);
                code = self.chess.handler(Crd::create(x, y));
            }
            
            if let Code::ChangePlayer = code {
                player = [1, 2][player%2];
            }
            if player == 1 { 
                println!("{}", match code {
                    Code::IncorrectCrd => "incorrect crd",
                    Code::NonePiece => "none piece",
                    Code::NoneState => "none state",
                    Code::NoneMoves => "none moves",
                    Code::ReselectPiece => "ReselectPiece",
                    Code::Exit => "exit",
                    Code::None => "none",
                    Code::ChangePlayer => "Change player",
                }); 
            }
            
        
        }
    }


    fn game_move(&mut self) -> Code {
        println!();
        self.print_color_board();
        println!();

        println!("Enter coordinates: ");
        let args = self.get_vec_crd();

        self.clear_screen();
        if args.len() == 1 {
            if args[0] == "exit" || args[0] == "Exit" {
                return Code::Exit;
            }
            else{
                self.println_error("Not the right number of arguments. Two arguments are needed");
            }
        } else if args.len() == 2 {
            match self.args_to_usize(&args) {
                Some(n) => {
                        return self.chess.handler(Crd::create(n.0 as isize, n.1 as isize));
                        
                    
                }
                None => {
                    self.println_error("No number or number too large");
                    return Code::None;
                }
            }

        } else {
            self.println_error("There are too many arguments"); 
        }
        Code::None
    }

    fn _print_board(&self){
        print!("{:3}", " ");
        for i in 0..8 {
            print!("{i:4}");
        }
        println!();
        println!("{:<3}{:-<32}"," " ,"-");

        for (i, arr) in self.chess.get_board_usize().iter().enumerate() {
            print!(" {i:<2}|");
            for item in arr {
                
                print!("{:3} ", item);
            }
            println!();
        }
    }

    fn print_color_board(&self) {
        print!("{:4}", " ");
        for i in 0..8 {
            print!("{i:4}");
        }
        println!();
        println!("{:<3}{:-<32}"," " ,"-");

        for (i, arr) in self.chess.get_color_board().iter().enumerate() {
            print!(" {i:<2}|");
            for (a, b) in arr {
                if *b == 1 {
                    print!("{:2} {}"," " ,ConsoleGame::posible_move(&a.to_string()));
                } else {
                    print!("{:4}", a);
                }
                
            }
            println!();
        }
    }

    fn args_to_usize(&self, v: &Vec<String>) -> Option<(usize,usize)> {
        let mut numbers: [usize; 2] = [0, 0];

        for (i, item) in v.iter().enumerate(){
            match item.parse::<usize>() {
                Ok(n) => {
                    numbers[i] = n;
                } 
                Err(_) => {
                    return None;
                }
            }
        }
        return Some( (numbers[0], numbers[1]) )
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
    fn menu(&self) -> usize {
        
        println!("Menu:");
        println!("1 - play");
        println!("2 - bot");
        println!("3 - exit");
        println!("Enter the item: ");

        self.select_menu()
    }




    fn select_menu(&self) -> usize {
        
        let mut input = String::new();
        self.clear_screen();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {

                let string = input.trim();
                match string.parse::<usize>() {
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

    fn _white_piece(s: &String) -> String {
        "\x1B[30m\x1B[47m".to_string() + s + "\x1B[0m"
    }

    fn _black_piece(s: &String) -> String {
        "\x1B[30m\x1B[47m".to_string() + s + "\x1B[0m"
    }

    fn posible_move(s: &String) -> String {
        "\x1B[42;1m".to_string() + s + "\x1B[0m"
    }
}

