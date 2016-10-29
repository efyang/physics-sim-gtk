extern crate gtk;
extern crate cairo;
extern crate physics_sim;

mod uistate;
mod editstate;
mod ui;

use ui::{Ui, IterationResult};

fn main() {
    let mut ui = Ui::initialize();
    loop {
        match ui.iterate() {
            IterationResult::Ok => {}
            IterationResult::Finished => break,
            IterationResult::Error(message) => println!("ERROR: {:?}", message),
        }
    }
}
