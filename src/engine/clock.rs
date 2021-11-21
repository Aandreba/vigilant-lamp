use std::time::{Duration, Instant};

pub struct Clock {
    last_call: Instant
}

impl Clock {
    pub fn new () -> Clock {
        Clock { last_call: Instant::now() }
    }

    pub fn delta (&mut self) -> Duration {
        let now = Instant::now();
        let dt = now - self.last_call;

        self.last_call = now;
        dt
    }
}