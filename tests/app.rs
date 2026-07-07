use Colony_Builder::core::CorePlugin;
use Colony_Builder::core::state::GameState;
use Colony_Builder::simulation::SimulationPlugin;
use Colony_Builder::world::WorldPlugin;
use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;

fn minimal_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()));
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins((CorePlugin, SimulationPlugin, WorldPlugin));
    app
}

#[test]
fn app_starts_in_loading_state() {
    let app = minimal_app();
    assert_eq!(
        *app.world().resource::<State<GameState>>(),
        GameState::Loading
    );
}

#[test]
fn app_transitions_to_playing() {
    let mut app = minimal_app();
    app.update();
    app.update();
    assert_eq!(
        *app.world().resource::<State<GameState>>(),
        GameState::Playing
    );
}

#[test]
fn app_has_tick_resource() {
    let app = minimal_app();
    assert!(
        app.world()
            .get_resource::<Colony_Builder::core::time::TickResource>()
            .is_some()
    );
}

#[test]
fn app_has_tile_grid() {
    let app = minimal_app();
    assert!(
        app.world()
            .get_resource::<Colony_Builder::world::TileGrid>()
            .is_some()
    );
}
