use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy)]
pub struct TickEvent {
    pub tick: u64,
    pub day: u64,
    pub tick_of_day: u64,
}

pub struct CoreEventPlugin;

impl Plugin for CoreEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TickEvent>();
    }
}
