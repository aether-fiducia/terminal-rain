use rand::prelude::*;

use termion;
use termion::raw::{IntoRawMode, RawTerminal};

use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem;
use std::io::{stdout, Stdout, Write};

// This should be self explanitory bud
const CHARS: &str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNMｦｧｨｩｪｫｬｭｮｯｰｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ1234567890-=*_+|:<>";

// The main app that will need two methods, new and update.
pub struct MatrixApp {
    columns: Vec<Column>,
    stdout: RefCell<RawTerminal<Stdout>>,
}

// Column struct to hold releven tinformation about the current stream of characters
struct Column {
    row_count: u16,
    wait_time: u16,
    rng: ThreadRng,
    is_drawing: bool,
    // Use a node type that can store information on the current and previous characters
    // it also needs a y pos and should either be a writer or eraser
    nodes: VecDeque<Node>,
}

// Node structure for storing current, previous, and position of characters
// Also stores the mode as either Writer or Eraser
struct Node {
    node_type: NodeType,
    y: u16,
    previous_char: Character,
    char: Character,
}

// Define writer/eraser
enum NodeType {
    Eraser,
    // Default color to white by default
    Writer { white: bool, rng:ThreadRng}
}

// Define character enum of either advanced char struct or empty
enum Character {
    Char { // More options could be defined if it was wanted...
        char: char,
        bold: bool,
        color_type: ColorType,
    },
    Blank,
}

// Manage the color of either white or default (because I'm a lazy motherfucker)
enum ColorType {
    White,
    Normal,
}

impl NodeType {
    // Generate a random character if given a writer, Character::Blank otherwise
    fn choice_char(&mut self) -> Character {
        match self {
            // Only ref rng because match statements are more greed than god damn closures
            NodeType::Writer { white, ref mut rng } => {
                let char = String::from(CHARS).
                    chars().
                    collect::<Vec<char>>().
                    choose(rng).
                    unwrap().
                    to_owned();
                let bold = rng.gen();
                let color_type = if *white {
                    ColorType::White
                } else {
                    ColorType::Normal
                };
                Character::Char {
                    char,
                    bold,
                    color_type,
                }
            }
            NodeType::Eraser => Character::Blank,
        }
    }
}

// Node impl will have a new method and update
// See update method for more information
impl Node {
    fn new(mut node_type: NodeType) -> Node {
        let y = 1;
        let char = node_type.choice_char();
        Node {
            node_type,
            y,
            previous_char: Character::Blank,
            char,
        }
    }
    /* Alright, this is why this shit works.
     * First things first, the y value needs to increase by one to represent the character stream
     * moving doen the terminal.
     * Then, a new character needs to be made for the next_char in the stream.
     * The previous char needs to be replaced by the current_char and the current_char needs
     * the next_char.
     * To avoid clone() hell (even though they all do implement copy, so it probably wouldn't be
     * too bad) I'm just going to use mem::replace()
     */
    fn update(&mut self) {
        self.y += 1;
        let next_char = self.node_type.choice_char();
        self.previous_char = mem::replace(&mut self.char, next_char);
    }
}


