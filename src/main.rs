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

use gtk::prelude::*;
use ui::{Ui, IterationResult};

fn main() {
    initialize_gtk();

    let mut ui = Ui::initialize();

    gtk::timeout_add(50, move || {
        let mut continue_state = true;
        match ui.iterate() {
            IterationResult::Ok => {}
            IterationResult::Finished => {
                continue_state = false;
            },
            IterationResult::Error(message) => {
                continue_state = false;
                println!("ERROR: {:?}", message);
            },
        }
        Continue(continue_state)
    });

    gtk::main();
}

fn initialize_gtk() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
}
