use std::{time::{Duration}};
use instant::Instant;

/// Usefull util to keep track of time deltas
#[derive(Clone, Copy)]
pub struct Clock (Instant);

impl Clock {
    pub fn new () -> Clock {
        Clock(Instant::now())
    }

    pub fn delta (&mut self) -> Duration {
        let now = Instant::now();
        let dt = now - self.0;

        self.0 = now;
        dt
    }
}