use crate::core::time::TickResource;
use bevy::prelude::*;

pub fn print_tick(tick: Res<TickResource>) {
    println!(
        "Dia {}, tick {}/{} ({:.1}%)",
        tick.current_day() + 1,
        tick.tick_of_day(),
        TickResource::TICKS_PER_DAY,
        tick.day_progress() * 100.0,
    );
}
