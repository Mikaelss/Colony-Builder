use crate::core::event::TickEvent;
use crate::core::time::TickResource;
use bevy::prelude::*;

pub fn advance_tick(mut tick: ResMut<TickResource>, mut writer: MessageWriter<TickEvent>) {
    if tick.paused {
        return;
    }
    tick.tick_count += 1;
    tick.accumulator = tick.tick_count as f64 * TickResource::TICK_DURATION;
    writer.write(TickEvent {
        tick: tick.tick_count,
        day: tick.current_day(),
        tick_of_day: tick.tick_of_day(),
    });
}
