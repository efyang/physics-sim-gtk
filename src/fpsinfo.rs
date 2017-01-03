use time::precise_time_s;

pub const DEFAULT_FPS: f64 = 60.;

#[derive(Clone)]
pub struct FpsInfo {
    next_update: f64,
    update_time: f64,
}

impl Default for FpsInfo {
    fn default() -> FpsInfo {
        let update_time = 1. / DEFAULT_FPS;
        FpsInfo {
            next_update: precise_time_s() + update_time,
            update_time: update_time,
        }
    }
}

impl FpsInfo {
    pub fn should_redraw(&self) -> bool {
        precise_time_s() >= self.next_update
    }

    pub fn update_time(&mut self) {
        self.next_update = precise_time_s() + self.update_time
    }
}
