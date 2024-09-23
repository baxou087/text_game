use crate::{letter_pool::LetterPool, word_pool::WordPool};

const STARTING_CURRENCY:    u32 = 4;
const MAX_CURRENCY:         u32 = 20;


pub struct Game {
    word:       String,
    lp:         LetterPool,
    wp:         WordPool,
    currency:   u32,
    found:      u32
}


impl Game {

    /// Constructor for the main structure of the game
    pub fn new(filepath: String) -> Game {
        Game {
            word:       String::new(),
            lp:         LetterPool::new(),
            wp:         WordPool::new(filepath),
            currency:   STARTING_CURRENCY,
            found:      0
        }
    }

    /// Main loop of the game
    pub fn run(self: &mut Self) {
        loop {
            self.word = self.wp.get_word();
            self.display_game();

            let input = self.ask_for_user_input();
            if input == "QQ" { break; }
        }
    }


    ///Displays the game to the player
    pub fn display_game(self: &Self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
        println!("Words found   : {}", self.found);
        println!("Currency      : {} / {} (Win the game by getting 20 or more)", self.currency, MAX_CURRENCY);
        println!("");
        println!("Word to find  : {}", self.display_word());
        println!("Word to find  : {}", self.word);
        println!("");
        println!("Bought        : {}", LetterPool::display_letter_pool(self.lp.bought()));
        println!("Available     : {}", LetterPool::display_letter_pool(self.lp.available()));
    }


    /// Displays a word on the terminal depending on
    /// the letters that have been bought by the player
    /// 
    /// If the word contains the '-' character, then it is displayed as is
    /// 
    pub fn display_word(self: &Self) -> String {
        let mut s: String = String::from("");
        let bought: &Vec<char> = self.lp.bought();
        for c in self.word.chars() {
            if c == '-' {
                s.push_str(&String::from(" - "));
            } else if bought.contains(&c) {
                s.push_str(&String::from(c));
                s.push_str(&String::from(" "));
            } else {
                s.push_str(&String::from("_ "));
            }
        }

        s
    }


    /// Asks the player to either buy a letter (A to Z) or input "QQ" to quit the game
    ///
    /// If the player inputs anything else, then the game will loop until a valid
    /// input is passed
    pub fn ask_for_user_input(&self) -> String {
        use text_io::read;

        let mut input: String;

        loop {
            print!("What letter do you wish to buy (input QQ to quit the game) ? ");
            input = read!();
            input = input.to_uppercase();

            // quitting the game
            if input == "QQ" {
                break;
            }

            // valid input from the player
            let valid_input: bool = input.len() == 1 &&
                            (input.chars().nth(0).unwrap() >= 'A' &&
                             input.chars().nth(0).unwrap() <= 'Z');

            if valid_input {
                break;
            }
        }

        input
    }



}


//#[cfg(test)]
//mod tests {
//use crate::game::Game;
//
//}
