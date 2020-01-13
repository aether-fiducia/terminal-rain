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
                let character = String::from(CHARS).
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
                    char: character,
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
        let character = node_type.choice_char();
        Node {
            node_type,
            y,
            previous_char: Character::Blank,
            char: character,
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

// New, spawn, update, see documentation for each
impl Column {
    fn new(row_count: u16) -> Column {
        let mut rng = thread_rng();
        // This is arbitrary
        let wait_time = rng.gen_range(0, row_count);
        Column {
            row_count,
            wait_time,
            rng,
            nodes: VecDeque::new(),
            is_drawing: false,
        }
    }

    // Spawn node does what the name would imply, with a delay being set to the wait time and the
    // spawned node has a type opposite of the value of is_drawing
    fn spawn_node(&mut self) -> Node {
        // The following three lines are sort of arbitrary
        let max_range = self.row_count - 3;
        let start_delay = self.rng.gen_range(1, max_range);
        self.wait_time = start_delay;

        self.is_drawing = !self.is_drawing;
        if self.is_drawing {
            let white = self.rng.gen::<bool>();
            Node::new(NodeType::Writer {
                white,
                rng: thread_rng(),
            })
        } else {
            Node::new(NodeType::Eraser)
        }
    }

    // The update method needs to call update on every node
    // As well, if the wait time has reached zero a new node should be pushed to the back
    // Lastly, if the front node has hit the y max, it should be pop'd
    fn update(&mut self) {
        self.nodes.iter_mut().for_each(|n| n.update());

        if self.wait_time == 0 {
            let new_node_spawn = self.spawn_node();
            self.nodes.push_back(new_node_spawn);
        }
        else {
            self.wait_time -= 1;
        }

        if let Some(node) = self.nodes.front() {
            if node.y > self.row_count {
                self.nodes.pop_front();
            }
        }
    }
}

// Now, I am using a RefCell as of the minute of rthe RawTerminal but I am not sure this is smart.
// There shouldn't be a reason not to use it, but this is a message to future aether, if some
// threading bullshit comes up figure out a better way to do this
impl MatrixApp {
    
    pub fn new() -> MatrixApp {
        let (size_x, size_y) = termion::terminal_size().unwrap();
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
        // Totally arbitrary, thanks Ryan.
        let column_count = size_x / 2;

        let columns = (0..column_count).map(|_| Column::new(size_y)).collect(); 

        MatrixApp {
            columns,
            stdout: RefCell::new(stdout),
        }
    }

    // A MatrixApp will need to update on a tick and draw itself, so let's bundle that
    pub fn on_tick(&mut self) {
        self.update();
        self.draw()
    }

    // Update needs to update each column
    fn update(&mut self) {
        self.columns.iter_mut().for_each(|x| x.update());
    }

    fn draw(&mut self) {
        // First, each node in every column needs to have a {} placed in stdout with cursor
        // movement
        // Next, match against the character type to print either a character or a blank space
        // where the cursor now lies
        for (x, column) in self.columns.iter().enumerate() {
            for node in column.nodes.iter() {
                write!(
                    self.stdout.borrow_mut(),
                    "{}",
                    termion::cursor::Goto((x * 2) as u16, node.y)
                    )
                    .unwrap();

                match &node.char {
                    Character::Char{
                        char,
                        bold,
                        color_type,
                    } => {
                        match color_type {
                            ColorType::White => {
                                self.set_white_char_style();
                            }
                            ColorType::Normal => {
                                self.set_normal_char_style(*bold);
                            }
                        };
                        write!(
                            self.stdout.borrow_mut(),
                            "{}{}",
                            char,
                            termion::style::Reset
                            )
                            .unwrap();
                    }
                    Character::Blank => {
                        write!(self.stdout.borrow_mut(), " ").unwrap();
                    }
                }
                // Make a node.y of 1 skip
                if node.y == 1 {
                    continue;
                }

                // Only the first of the falling chars should be white
                if let Character::Char {
                    char,
                    bold,
                    color_type: ColorType::White,
                } = &node.char {
                    self.set_normal_char_style(*bold);
                    write!(
                        self.stdout.borrow_mut(),
                        "{}{}{}",
                        termion::cursor::Goto((x * 2) as u16, (node.y - 1) as u16),
                        char,
                        termion::style::Reset
                        )
                        .unwrap();
                }
            }
        }
        self.stdout.borrow_mut().flush().unwrap();
    }

    fn set_white_char_style(&self) {
        write!(
            self.stdout.borrow_mut(),
            "{}{}",
            termion::style::Bold,
            termion::color::Fg(termion::color::White)
            )
            .unwrap();
    }

    fn set_normal_char_style(&self, bold: bool) {
        if bold {
            write!(self.stdout.borrow_mut(), "{}", termion::style::Bold).unwrap();
        }
        write!(
            self.stdout.borrow_mut(),
            "{}",
            termion::color::Fg(termion::color::Green)
            )
            .unwrap();
    }
}

// Shit I needed this a while ago
impl Drop for MatrixApp {
    fn drop(&mut self) {
        write!(
            self.stdout.borrow_mut(), 
            "{}{}", 
            termion::clear::All, 
            termion::cursor::Show)
            .unwrap();
    }
}
