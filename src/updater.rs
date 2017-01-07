use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use coloruniverse::ColorUniverse;
use iteration_result::IterationResult;
//use physics_sim::Object;
use physics_sim::*;
use color::ObjectColor;

// if UI is finished first, then first send kill signal to updater, then this should automatically
// be stopped/dropped anyways

// add should_update variable or receiver which should be false when uistate is edit

pub struct Updater {
    update_send: Sender<ColorUniverse>,
    update_command_recv: Receiver<UpdaterCommand>,
    update_settings: UpdateSettings,
    universe: ColorUniverse,
    paused: bool,
    fps_update_time: f64,
}

impl Updater {
    pub fn new(universe: ColorUniverse) -> (Updater, Receiver<ColorUniverse>, Sender<UpdaterCommand>) {
        //let mut universe = universe.clone();
        //universe.add_object(Object::new(200000., Vector::default(), Point::new(0., 0.)), ObjectColor::FromMass);
        //universe.add_object(Object::new(10000., Vector::new(120., 0.), Point::new(0., 5000.)), ObjectColor::FromMass);
        //universe.add_object(Object::new(1., Vector::new(53., 0.), Point::new(0., 140.)), ObjectColor::FromMass);

        let (update_send, update_recv) = channel();
        let (update_command_send, update_command_recv) = channel();
        (Updater {
            update_send: update_send,
            update_command_recv: update_command_recv,
            update_settings: UpdateSettings::default(),
            universe: universe,
            paused: false,
            fps_update_time: 1./60.,
        },
        update_recv,
        update_command_send)
    }

    // WIP/TODO
    pub fn iterate(&mut self) -> IterationResult {
        let start_time = ::time::precise_time_s();
        // check for any new settings
        match self.update_command_recv.try_recv() {
            Ok(command) => {
                match command {
                    UpdaterCommand::UpdateSettings(new_settings) => {
                        self.update_settings = new_settings;
                    }
                    UpdaterCommand::Pause => {
                        self.paused = true;
                    }
                    UpdaterCommand::Unpause => {
                        self.paused = false;
                    }
                    UpdaterCommand::SetFpsUpdateTime(update_time) => {
                        self.fps_update_time = update_time;
                    }
                    UpdaterCommand::SetUniverse(universe) => {
                        self.universe = universe;
                    }
                }
            },
            Err(TryRecvError::Empty) => {},
            Err(e) => return IterationResult::Error(format!("{}", e)),
        }
        // update the universe
        if !self.paused {
            self.universe
                .update_state_repeat(self.update_settings.time, self.update_settings.iterations);

            // write out the new universe to the ringbuffer
            if let Err(_) = self.update_send.send(self.universe.clone()) {
                return IterationResult::Finished;
            }
        }

        let time_taken = ::time::precise_time_s() - start_time;
        // OVERFLOW
        let time_sleep = self.fps_update_time - time_taken;
        if time_sleep > 0. {
            ::std::thread::sleep(::std::time::Duration::from_millis((time_sleep * 1000.) as u64));
        }
        // continue
        IterationResult::Ok
    }
}

pub enum UpdaterCommand {
    UpdateSettings(UpdateSettings),
    Pause,
    Unpause,
    SetFpsUpdateTime(f64),
    SetUniverse(ColorUniverse),
}

pub struct UpdateSettings {
    time: f64,
    iterations: usize,
}

impl Default for UpdateSettings {
    fn default() -> UpdateSettings {
        UpdateSettings {
            time: 3000.,
            iterations: 100,
        }
    }
}

impl UpdateSettings {
    pub fn time(&self) -> f64 {
        self.time
    }
}
