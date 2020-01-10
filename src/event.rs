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

/* Make events struct with a Receiver for Event(s) an input handler and tick handler.
 * The receiver wiill moniter a mpsc channel with thhe sender being used by the handler threads.
 * The input thread waits for a Ctrl+C from the user.
 * The tick thread moniters the tick rate.
 */
pub struct Events {
    rx: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandler<()>,
    tick_handler: thread::JoinHandler<()>,
}

impl Events {
    pub fn new(tick_rate: u64) -> Self {
        // Open a sender and receiver for the Events receiver and two threads
        let (tx, rx) = mpsc::channel();
        /* Make the input handler by:
         * One-> Cloning the sending channel
         * Two-> Spawning a new thread
         * Three-> Seeing if Crtl+C was pressed by the user
         * (Four)-> Sending the Exit Event should that occur
         */
        let input_handler = {
            let tx = tx.clone();
            thread::spawn (move || {
                let stdin = io::stdin();
                if stdin.keys().any(|x| x == Ok(Key::Crtl('c'))) {
                    tx.send(Event::Exit).unwrap();
                }
            })
        };
        /* Make the tick handler by:
         * One-> Cloning the sending channel
         * Two-> Spawning a new looping thread (loops for duration of main thread)
         * Three-> Sending a Tick Event to the Receiver
         * Four-> Pausing for the supplied tick rate amount of time
         */
        let tick_handler = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                tx.send(Event.Tick).unwrap();
                thread::sleep(Duration::from_millis(tick_rate));
            })
        };
        // Return Events
        Events {
            rx,
            input_handler: input_handler,
            tick_handler: tick_handler,
        }
    }
    // Call next function on hidden reveiver
    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}

