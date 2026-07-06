use bevy::prelude::*;

#[derive(Resource)]
pub struct TickResource {
    pub tick_count: u64,
    pub accumulator: f64,
    pub paused: bool,
}

impl TickResource {
    pub const TICKS_PER_DAY: u64 = 60_000;
    pub const DAY_DURATION_SECS: f64 = 1000.0;
    pub const TICK_DURATION: f64 = Self::DAY_DURATION_SECS / Self::TICKS_PER_DAY as f64;

    pub fn current_day(&self) -> u64 {
        self.tick_count / Self::TICKS_PER_DAY
    }

    pub fn tick_of_day(&self) -> u64 {
        self.tick_count % Self::TICKS_PER_DAY
    }

    #[allow(dead_code)]
    pub fn day_progress(&self) -> f64 {
        self.tick_of_day() as f64 / Self::TICKS_PER_DAY as f64
    }
}

impl Default for TickResource {
    fn default() -> Self {
        Self {
            tick_count: 0,
            accumulator: 0.0,
            paused: false,
        }
    }
}
