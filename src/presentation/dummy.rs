use crate::core::event::TickEvent;
use crate::core::time::TickResource;
use bevy::prelude::*;

pub fn print_tick(mut reader: MessageReader<TickEvent>) {
    for ev in reader.read() {
        println!(
            "Dia {}, tick {}/{} ({:.1}%)",
            ev.day + 1,
            ev.tick_of_day,
            TickResource::TICKS_PER_DAY,
            ev.tick_of_day as f64 / TickResource::TICKS_PER_DAY as f64 * 100.0,
        );
    }
}
