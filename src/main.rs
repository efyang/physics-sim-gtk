#![feature(conservative_impl_trait)]
#![allow(dead_code)]
extern crate gtk;
extern crate cairo;
extern crate physics_sim;
extern crate time;

mod uistate;
mod editstate;
mod ui;
mod sharedstate;
mod fpsinfo;
mod drawinfo;
mod drawobject;
mod color;
mod updater;
mod coloruniverse;

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
