extern crate termion;
extern crate rand;

use rand::Rng;

use std::convert::TryFrom;

use std::io::{self, stdout, Read, Write};

use termion::{clear, cursor, raw::IntoRawMode, async_stdin};

use std::thread;

use std::process;

use std::time::Duration;

// A structure to hold the character buffers and what parts are visable
#[derive(Clone)]
struct CharStream {
    pub sequence: String,
    pub visable: (u16, u16),
    pub pos: u16,
}

impl CharStream {

    // Gen a new full CharStream given a length
    fn new(length: u16) -> Self {
        let mut result = Self {sequence: String::new(), visable: (0,0), pos: 0};
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
    // Most hard coded numbers are arbitrary, notes will be made for important ones
    let mut rng = rand::thread_rng();
    let (mut x_tty_size, mut y_tty_size): (u16, u16) = termion::terminal_size().unwrap();
    let mut current = vec![CharStream::new(rng.gen_range(1, y_tty_size / 2) as u16); rng
        .gen_range(x_tty_size / 8, x_tty_size / 3) as usize];
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    // Program loops until user input recived
    loop {
        
        write!(stdout, "\n{}{}\n", cursor::Hide, clear::All).unwrap();
        current.clone().into_iter();

    }

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
