use rand::prelude::*;

use termion;
use termion::raw::{IntoRawMode, RawTerminal};

use std::cell:RefCell;
use std::collections::VecDeque;
use std::mem;

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
    Writer { white: bool, rng: Threadrng}
}

// Define character enum of either advanced char struct or empty
enum Character {
    Char { // More options could be defined if it was wanted...
        char: char,
        bold: bool,
        color_type: ColorType,
    },
    Empty,
}

// Manage the color of either white or default (because I'm a lazy motherfucker)
enum ColorType {
    White,
    Normal,
}

impl NodeType {
    // Generate a random character if given a writer, Character::Blank otherwise
    fn chioce_char(&mut self) -> Character {
        match self {
            // Only ref rng because match statements are more greed than god damn closures
            NodeType::Writer { white, ref mut rng } => {
                let chars = String::from(CHARS).chars().coolect::<Vec<char>>();
                let char = chars.choose(rng).unwrap().to_owned();
                let bold = rng.gen();
                let color_type = if *white {
                    ColorType::White
                } else {
                    ColorType::Normal
                };
                Character {
                    char,
                    bold,
                    color_type,
                }
            }
            NodeType::Eraser => Character::Blank,
        }
    }
}

