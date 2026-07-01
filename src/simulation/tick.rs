use crate::core::time::TickResource;
use bevy::prelude::*;

pub fn advance_tick(mut tick: ResMut<TickResource>) {
    if tick.paused {
        return;
    }
    tick.tick_count += 1;
    tick.accumulator = tick.tick_count as f64 * TickResource::TICK_DURATION;
}
