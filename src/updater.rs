use std::sync::mpsc::{Sender, Receiver};
use coloruniverse::ColorUniverse;

// if UI is finished first, then first send kill signal to updater, then this should automatically
// be stopped/dropped anyways

// add should_update variable or receiver which should be false when uistate is edit

pub struct Updater {
    update_output: Sender<ColorUniverse>,
    update_settings_recv: Receiver<UpdateSettings>,
    update_settings: UpdateSettings,
    universe: ColorUniverse,
}

impl Updater {
    fn new(update_output: Sender<ColorUniverse>,
           update_settings_recv: Receiver<UpdateSettings>,
           universe: ColorUniverse)
           -> Updater {
        Updater {
            update_output: update_output,
            update_settings_recv: update_settings_recv,
            update_settings: UpdateSettings::default(),
            universe: universe,
        }
    }

    // WIP/TODO
    fn iterate(&mut self) {
        // check for any new settings
        
        // update the universe
        self.universe
            .update_state_repeat(self.update_settings.time, self.update_settings.iterations);
        
        // write out the new universe to the ringbuffer
        self.update_output.send(self.universe.clone()).unwrap();
    }
}

pub struct UpdateSettings {
    time: f64,
    iterations: usize,
}

impl Default for UpdateSettings {
    fn default() -> UpdateSettings {
        UpdateSettings {
            time: 1.,
            iterations: 10,
        }
    }
}
