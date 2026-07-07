use Colony_Builder::core::event::TickEvent;
use Colony_Builder::core::state::GameState;
use Colony_Builder::core::time::TickResource;
use Colony_Builder::simulation::tick::advance_tick;
use bevy::prelude::*;

#[test]
fn advance_tick_does_not_run_during_loading() {
    let mut app = App::new();
    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_once());
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.init_resource::<TickResource>();
    app.init_state::<GameState>();
    app.add_message::<TickEvent>();
    app.add_systems(Update, advance_tick.run_if(in_state(GameState::Playing)));

    app.update();
    let tick = app.world().resource::<TickResource>();
    assert_eq!(tick.tick_count, 0);
}

#[test]
fn advance_tick_increments_count_directly() {
    let mut app = App::new();
    app.init_resource::<TickResource>();
    app.add_message::<TickEvent>();
    app.add_systems(Update, advance_tick);

    app.update();
    let tick = app.world().resource::<TickResource>();
    assert_eq!(tick.tick_count, 1);
}

#[test]
fn advance_tick_increments_accumulator() {
    let mut app = App::new();
    app.init_resource::<TickResource>();
    app.add_message::<TickEvent>();
    app.add_systems(Update, advance_tick);

    app.update();
    let tick = app.world().resource::<TickResource>();
    let expected = 1.0 * TickResource::TICK_DURATION;
    assert!((tick.accumulator - expected).abs() < 1e-10);
}
