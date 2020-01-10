use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

// Define event enum for events struct with reciever
pub enum Event {
    Tick,
    Exit,
}

/* Make events struct with a Receiver for Event(s) an input handle and tick handle.
 * The receiver wiill moniter a mpsc channel with thhe sender being used by the handle threads.
 * The input thread waits for a Ctrl+C from the user.
 * The tick thread moniters the tick rate.
 */
pub struct Events {
    rx: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new(tick_rate: u64) -> Self {
        // Open a sender and receiver for the Events receiver and two threads
        let (tx, rx) = mpsc::channel();
        /* Make the input handle by:
         * One-> Cloning the sending channel
         * Two-> Spawning a new thread
         * Three-> Seeing if Crtl+C was pressed by the user
         * (Four)-> Sending the Exit Event should that occur
         */
        let input_handle = {
            let tx = tx.clone();
            thread::spawn (move || {
                let stdin = io::stdin();
                if stdin.keys().any(|x| if let Ok(Key::Ctrl('c')) = x {true}
                                    else {false}) {
                    tx.send(Event::Exit).unwrap();
                }
            })
        };
        /* Make the tick handle by:
         * One-> Cloning the sending channel
         * Two-> Spawning a new looping thread (loops for duration of main thread)
         * Three-> Sending a Tick Event to the Receiver
         * Four-> Pausing for the supplied tick rate amount of time
         */
        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                tx.send(Event::Tick).unwrap();
                thread::sleep(Duration::from_millis(tick_rate));
            })
        };
        // Return Events
        Events {
            rx,
            input_handle: input_handle,
            tick_handle: tick_handle,
        }
    }
    // Call next function on hidden reveiver
    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}

