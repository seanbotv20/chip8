use std::cmp;
use std::thread;
use std::time::{Duration, Instant};

const DELAY_INTERVAL: Duration = Duration::from_secs(1);

pub struct MainTimer {
    hz: u16,
    last_tick: Instant,
}

impl MainTimer {
    pub fn new(hz: u16) -> MainTimer {
        return MainTimer {
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

pub struct DelayTimer {
    last_tick: Instant,
    value: u8,
}

impl DelayTimer {
    pub fn new() -> DelayTimer {
        return DelayTimer {
            last_tick: Instant::now(),
            value: 0,
        };
    }

    fn is_running(&self) -> bool {
        self.value > 0
    }

    pub fn set(&mut self, value: u8) {
        if !self.is_running() {
            self.last_tick = Instant::now();
        }
        self.value = value;
    }

    pub fn get(&self) -> u8 {
        return self.value;
    }

    pub fn update(&mut self) {
        if self.is_running() {
            let now = Instant::now();
            if now >= self.last_tick + DELAY_INTERVAL {
                let duration_since = now - self.last_tick;

                let whole_seconds = (duration_since.as_secs() / DELAY_INTERVAL.as_secs()) as u8;

                let next_tick = self.last_tick + DELAY_INTERVAL * (whole_seconds as u32);

                self.value -= cmp::min(self.value, whole_seconds);
                self.last_tick = next_tick;
            }
        }
    }
}
