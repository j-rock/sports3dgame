use dimensions::time::{
    DeltaTime,
    Microseconds
};

// For a given window duration, counts the number of windows that have passed
// since this Ticker was created.
pub struct Ticker {
    window_duration: Microseconds,
    microseconds_elapsed_since_window_start: Microseconds,
    num_windows_elapsed: usize,
}

impl Ticker {
    pub fn new(window_duration: Microseconds) -> Ticker {
        Ticker {
            window_duration,
            microseconds_elapsed_since_window_start: 0,
            num_windows_elapsed: 0
        }
    }

    pub fn update(&mut self, dt: DeltaTime) {
        self.microseconds_elapsed_since_window_start += dt.as_microseconds();
        while self.microseconds_elapsed_since_window_start > self.window_duration {
            self.microseconds_elapsed_since_window_start -= self.window_duration;
            self.num_windows_elapsed += 1;
        }
    }

    pub fn get_tick(&self) -> usize {
        self.num_windows_elapsed
    }

    pub fn current_window_completion_ratio(&self) -> f32 {
        self.microseconds_elapsed_since_window_start as f32 / self.window_duration as f32
    }

    pub fn clear(&mut self) {
        self.microseconds_elapsed_since_window_start = 0;
        self.num_windows_elapsed = 0;
    }
}