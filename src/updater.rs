use std::sync::mpsc::{Sender, Receiver};
use rb::Producer;

// if UI is finished first, then first send kill signal to updater, then this should automatically
// be stopped/dropped anyways

// add should_update variable or receiver which should be false when uistate is edit

pub struct Updater {
    error_output: Sender<String>,
    update_output: Producer<ColorUniverse>,
    internal_value: ColorUniverse,
}

impl Updater {
    fn new(error_output: Sender<String>,
           update_output: Producer<ColorUniverse>,
           universe: ColorUniverse)
        -> Updater {
            Updater {
                error_output: error_output,
                update_output: update_output,
                internal_value: universe,
            }
        }

    fn iterate(&mut self) {
        
    }
}
