pub mod dummy;

use bevy::prelude::*;
use dummy::print_tick;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_tick);
    }
}
