extern crate termion;
extern crate rand;

use rand::Rng;

// A structure to hold the character buffers and what parts are visable
struct CharStream {
    pub sequence: String,
    pub visable: (u16, u16),
}

impl CharStream {

    fn new(length: u16) -> Self {
        let mut result = Self {sequence: String::new(), visable: (0,0)};
        let mut rng = rand::thread_rng();
        for _k in 0..length {
            result.sequence.push(rng.gen());
        }
        return result;
    }

}

fn main() {
    println!("Well shit it works");
}

#[test]
fn test() {
    let test_bio = CharStream::new(2);
    println!("{}", test_bio.sequence);
    println!("{}", test_bio.visable.1);
}
