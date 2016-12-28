#![feature(conservative_impl_trait)]
#![allow(dead_code)]
extern crate gtk;
extern crate cairo;
extern crate physics_sim;
extern crate time;
extern crate gdk_sys;
extern crate gdk;

mod uistate;
mod editstate;
mod ui;
mod sharedstate;
mod fpsinfo;
#[macro_use]
mod color;
mod drawinfo;
mod drawobject;
mod updater;
mod coloruniverse;
mod iteration_result;

use gtk::prelude::*;
use ui::Ui;
use iteration_result::IterationResult;

fn main() {
    initialize_gtk();

    let mut ui = Ui::initialize();

    gtk::timeout_add(30, move || {
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
