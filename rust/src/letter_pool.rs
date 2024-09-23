const NB_LETTERS_IN_ALPHABET: usize = 26;
const CHAR_POOL: [char; NB_LETTERS_IN_ALPHABET] = ['A','B','C','D','E','F',
                                                   'G','H','I','J','K','L',
                                                   'M','N','O','P','Q','R',
                                                   'S','T','U','V','W','X',
                                                   'Y','Z'];

    /// The LetterPool structure will help us keep track of what
    /// letters have been bought, and what letters are still available
    /// for the player to buy
    pub struct LetterPool {
        available:  Vec<char>,
        bought:     Vec<char>
}


impl LetterPool {

    //! Basic constructor for the LetterPool object.
    //! 
    //! Creates and returns a vector of chars containing
    //! the alphabet.
    //! 
    //! # Example
    //! 
    //! ```
    //!     let lp: LetterPool = LetterPool::new();
    //!
    //!         for i in 0..NB_LETTERS_IN_ALPHABET {
    //!         assert_eq!(CHAR_POOL[i], lp.lp[i]);
    //!     }
    //!
    //! ```
    pub fn new() -> LetterPool {
        let mut alphabet: Vec<char> = Vec::with_capacity(NB_LETTERS_IN_ALPHABET);

        for c in CHAR_POOL {
            alphabet.push(c);
        }

        // Creating the letter_pool and returning it
        LetterPool {
            available:  alphabet,
            bought:     Vec::with_capacity(NB_LETTERS_IN_ALPHABET)
        }
    }



    /// #is_letter_available
    /// Method used to check if the letter the player wants to buy
    /// is available
    /// 
    /// This method won't check if the letter passed as an argument is a valid.
    /// The calling function will have to make sure the letter is valid.
    /// 
    /// # Example
    /// ```
    ///    let lp: LetterPool = LetterPool::new();
    ///
    ///    assert_eq!(lp.is_letter_available('a'), false);
    ///    assert_eq!(lp.is_letter_available('A'), true);
    /// ```
    pub fn is_letter_available(self: &Self, letter: char) -> bool {
        self.available.contains(&letter)
    }


    /// Method used to buy a letter.
    /// Before buying a letter we first have to make sure the letter is
    /// available for puchase.
    /// 
    /// This method won't check if the letter passed as an argument is a valid.
    /// The calling function will have to make sure the letter is valid.
    /// 
    /// # Example
    /// ```
    ///    let mut lp: LetterPool = LetterPool::new();
    ///
    ///    assert_eq!(lp.is_letter_available('A'), true);
    ///    lp.buy_letter('A');
    ///    assert_eq!(lp.is_letter_available('A'), false);
    ///
    ///    assert_eq!(lp.bought.contains(&'A'), true);
    /// ```
    pub fn buy_letter(self: &mut Self, letter: char) -> bool {
        if self.is_letter_available(letter) {
            let index = self.available.iter().position(|&r| r == letter).unwrap();
            self.bought.push(letter);
            self.available.remove(index);
            true
        } else {
            false
        }
    }

    /// Borrows the available vector to the caller
    pub fn available(self: &Self) -> &Vec<char> {
        &self.available
    }

    /// Borrows the bought vector to the caller
    pub fn bought(self: &Self) -> &Vec<char> {
        &self.bought
    }


    /// Displays the letter pool that has been passed as an argument
    /// 
    /// If the letter_pool is empty, the method returns "[]"
    /// 
    /// If the letter_pool contains A, B and C, it return "[ A ; B ; C ]"
    /// 
    /// # Example
    /// ```
    ///    let v1: Vec<char> = Vec::new();
    ///    assert_eq!(Game::display_letter_pool(v1), "[]");
    ///    let v2: Vec<char> = vec!['A', 'B', 'C'];
    ///    assert_eq!(Game::display_letter_pool(v2), "[ A ; B ; C ]");
    /// ```
    pub fn display_letter_pool(lp: &Vec<char>) -> String {
        if lp.len() == 0 {
            String::from("[]")
        } else {
            let sz = lp.len() - 1;

            let mut s: String = String::from("[ ");
            for (i, c) in lp.iter().enumerate() {
                s.push_str(&c.to_string());
                if i != sz {s.push_str(" ; ");}
            }
            s.push_str(" ]");

            s
        }
    }


}

#[cfg(test)]
mod tests {
    use super::{LetterPool, CHAR_POOL, NB_LETTERS_IN_ALPHABET};


    #[test]
    fn test_new() {
        let lp: LetterPool = LetterPool::new();

        for i in 0..NB_LETTERS_IN_ALPHABET {
            assert_eq!(CHAR_POOL[i], lp.available[i]);
        }
        assert_eq!(lp.bought.is_empty(), true);
    }



    #[test]
    fn test_is_letter_available() {
        let lp: LetterPool = LetterPool::new();

        assert_eq!(lp.is_letter_available('a'), false);
        assert_eq!(lp.is_letter_available('A'), true);
    }



    #[test]
    fn test_buy_letter() {
        let mut lp: LetterPool = LetterPool::new();

        assert_eq!(lp.is_letter_available('A'), true);
        lp.buy_letter('A');
        assert_eq!(lp.is_letter_available('A'), false);

        assert_eq!(lp.bought.contains(&'A'), true);
    }


    #[test]
    fn test_display_letter_pool() {
        let v1: Vec<char> = Vec::new();
        assert_eq!(LetterPool::display_letter_pool(&v1), "[]");
        let v2: Vec<char> = vec!['A', 'B', 'C'];
        assert_eq!(LetterPool::display_letter_pool(&v2), "[ A ; B ; C ]");
    }


}