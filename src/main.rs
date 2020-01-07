extern crate termion;
extern crate rand;

use rand::Rng;

use std::convert::TryFrom;

use std::io::{self, stdout, Read, Write};

use termion::{clear, cursor, raw};

use std::thread;

use std::process;

use std::time::Duration;

// A structure to hold the character buffers and what parts are visable
struct CharStream {
    pub sequence: String,
    pub visable: (u16, u16),
}

impl CharStream {

    // Gen a new full CharStream given a length
    fn new(length: u16) -> Self {
        let mut result = Self {sequence: String::new(), visable: (0,0)};
        let mut rng = rand::thread_rng(); // Not secure but fast rng 
        for _k in 0..length {
            result.sequence.push( // 4. Add random char to the sequence
                char::try_from( // 3. Use u32 as standard latin character
                    rng.gen_range(0x0020, 0x007F) as u32) // 1. Gen random <u32,error>
                .unwrap()); // 2. Discard error
        }
        return result;
    }

    // Increase max tail visability
    fn grow(&mut self) {
        self.visable.1 += 1;
    }

    //Decrease head visability
    fn shrink(&mut self) {
        self.visable.0 += 1;
    }

}

fn main() {
    let (x_tty_size, y_tty_size) = termion::terminal_size().unwrap();
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let mut test_bio = CharStream::new(2);
        test_bio.visable.1 = 18;
    }
}
