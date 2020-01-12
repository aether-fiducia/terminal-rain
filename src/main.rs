mod event;
mod app;

use crate::event::{Event, Events};
use crate::app::MatrixApp;

fn main() {
    let events = Events::new(250);
    let mut app = MatrixApp::new();

    loop {
        match events.next().unwrap() {
            Event::Tick => {
                app.on_tick();
            }
            Event::Exit => break,
        }
    }
}
