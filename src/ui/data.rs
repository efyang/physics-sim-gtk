use super::state::UiState;
use updater::{UpdateSettings, UpdaterCommand};
use coloruniverse::ColorUniverse;
use input::InputInfo;
use fpsinfo::*;
use std::sync::mpsc::{Sender, Receiver};
use draw::DrawInfo;

pub struct UiData {
    pub state: UiState,
    pub universe: ColorUniverse,
    pub fps_info: FpsInfo,
    pub draw_info: DrawInfo,
    pub input_info: InputInfo,
    pub universe_recv: Receiver<ColorUniverse>,
    pub update_settings: UpdateSettings,
    pub update_command_send: Sender<UpdaterCommand>,
    pub allow_mouse_movement: bool,
}

impl UiData {
    pub fn new(universe_recv: Receiver<ColorUniverse>,
               update_command_send: Sender<UpdaterCommand>)
               -> UiData {
        UiData {
            state: UiState::default(),
            universe: ColorUniverse::default(),
            fps_info: FpsInfo::default(),
            draw_info: DrawInfo::default(),
            universe_recv: universe_recv,
            update_settings: UpdateSettings::default(),
            update_command_send: update_command_send,
            input_info: InputInfo::default(),
            allow_mouse_movement: false,
        }
    }
}
