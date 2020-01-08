extern crate termion;
extern crate rand;

use rand::Rng;

use std::convert::TryFrom;

use std::io::{stdout, Read, Write};

use termion::{clear, cursor, raw::IntoRawMode, async_stdin};

use std::time::Duration;

use std::thread;

// A structure to hold the character buffers and what parts are visable
#[derive(Clone)]
struct CharStream {
    pub sequence: String,
    pub visable: (u16, u16),
    pub x_pos: u16,
    pub y_pos: u16,
}

impl CharStream {

    // Gen a new full CharStream given a length
    fn new(length: u16) -> Self {
        let mut result = Self {sequence: String::new(), visable: (0,0), x_pos: 0, y_pos: 0};
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

fn resized<W: Write>(size: (u16, u16), stdout: &mut termion::raw::RawTerminal<W>) -> bool {
    if size != termion::terminal_size().unwrap() {
        write!(stdout, "{}", clear::All).unwrap();
        true
    } else {
        false
    }
}

fn main() {
    // Most hard coded numbers are arbitrary, notes will be made for important ones
    // Start rng thread
    let mut rng = rand::thread_rng();
    // Get starting terminal size, but this might change due to fullscreen
    let mut size: (u16, u16) = termion::terminal_size().unwrap();
    // Generate some charstreams to start off with
    let mut current = vec![CharStream::new(rng.gen_range(1, size.1 / 2) as u16); rng
        .gen_range(size.0 / 8, size.0 / 3) as usize];
    // Format stdout and stdin threads
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    // Program loops until user input recived
    loop {
        // Clear terminal
        write!(stdout, "\n{}{}\n", cursor::Hide, clear::All).unwrap();
        // Check user input
        let mut exit = 0;
        let ev = stdin.next();
        if let Some(Ok(b)) = ev {
            match b {
                b'q' => {
                    exit = 1;
                    break;
                }
                _ => (),
            }
        }

        if exit == 1 {
            break
        }

        if resized(size, &mut stdout) {
            println!("\n\n\n\n\nYou resized the window, restarting the rain...");
            thread::sleep(Duration::new(5, 0));
            main();
        }

        current.clone().into_iter();

    }

    write!(stdout, "{}", termion::cursor::Show).unwrap(); //Reset cursor

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
