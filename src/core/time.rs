use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TickResource {
    pub tick_count: u64,
}
