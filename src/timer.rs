use std::thread;
use std::time::{Duration, Instant};

pub struct Timer {
    hz: u16,
    last_tick: Instant,
}

impl Timer {
    pub fn new(hz: u16) -> Timer {
        return Timer {
            hz,
            last_tick: Instant::now(),
        };
    }

    pub fn reset(&mut self) {
        self.last_tick = Instant::now()
    }

    pub fn wait_for_next_tick(&mut self) {
        let time_to_wait = Duration::from_nanos(100000000 / self.hz as u64);
        thread::sleep(time_to_wait);
        self.last_tick = Instant::now();
    }
}
