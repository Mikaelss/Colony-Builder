use crate::core::time::TickResource;
use bevy::prelude::*;

pub fn advance_tick(mut tick: ResMut<TickResource>) {
    tick.tick_count += 1;
}
