use crate::core::time::TickResource;
use bevy::prelude::*;

pub fn print_tick(tick: Res<TickResource>) {
    println!("Tick: {}", tick.tick_count);
}
