use sharedstate::SharedState;
use gdk::EventKey;
use gdk::enums::key;
use updater::UpdaterCommand;
use std::sync::mpsc::TryRecvError;
use coloruniverse::ColorUniverse;

use super::data::UiData;
use super::state::*;

pub fn key_press_handler(data: &SharedState<UiData>, key: &EventKey) {
    let ref mut data = *data.get_state_mut();
    match key.get_keyval() {
        key::Shift_L | key::Shift_R => {
            data.input_info.shift = true;
        }
        key::Control_L | key::Control_R => {
            data.input_info.ctrl = true;
        }
        key::Up => {
            data.input_info.up = true;
        }
        key::Down => {
            data.input_info.down = true;
        }
        key::Left => {
            data.input_info.left = true;
        }
        key::Right => {
            data.input_info.right = true;
        }
        _ => {
            println!("keypressed");
        }
    }
}

pub fn key_release_handler(data: &SharedState<UiData>, key: &EventKey) {
    let ref mut data = *data.get_state_mut();
    match key.get_keyval() {
        key::P | key::p => {
            let new_state = match data.state {
                UiState::Paused => {
                    data.update_command_send
                        .send(UpdaterCommand::Unpause)
                        .unwrap();
                    UiState::Normal
                }
                _ => {
                    data.update_command_send
                        .send(UpdaterCommand::Pause)
                        .unwrap();
                    UiState::Paused
                }
            };
            data.state = new_state;
        }
        key::E | key::e => {
            let new_state = match data.state {
                UiState::Edit(_) => {
                    data.update_command_send
                        .send(UpdaterCommand::Unpause)
                        .unwrap();
                    UiState::Normal
                }
                _ => {
                    data.update_command_send
                        .send(UpdaterCommand::Pause)
                        .unwrap();
                    UiState::Edit(EditState::default())
                }
            };
            data.state = new_state;
        }
        key::R | key::r => {
            data.universe = ColorUniverse::default();
            data.update_command_send
                .send(UpdaterCommand::SetUniverse(data.universe.clone()))
                .unwrap();
            // clear receiver
            let mut clear = false;
            while !clear {
                match data.universe_recv.try_recv() {
                    Ok(_) => {}
                    Err(TryRecvError::Empty) => clear = true,
                    Err(e) => println!("error: {:?}", e),
                }
            }
        }
        key::Shift_L | key::Shift_R => {
            data.input_info.shift = false;
        }
        key::Control_L | key::Control_R => {
            data.input_info.ctrl = false;
        }
        key::Up => {
            data.input_info.up = false;
        }
        key::Down => {
            data.input_info.down = false;
        }
        key::Left => {
            data.input_info.left = false;
        }
        key::Right => {
            data.input_info.right = false;
        }
        key::BackSpace => {
            let ref mut backspace = data.input_info.backspace;
            backspace.next_state();
            if backspace.should_reset() {
                data.draw_info.reset_view();
                backspace.next_state();
            }
        }
        key::M | key::m => {
            data.allow_mouse_movement = !data.allow_mouse_movement;
        }
        _ => {
            println!("keypress");
        }
    }
}
