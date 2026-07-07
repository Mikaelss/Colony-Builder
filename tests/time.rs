use Colony_Builder::core::time::TickResource;

#[test]
fn default_tick_count_zero() {
    let tick = TickResource::default();
    assert_eq!(tick.tick_count, 0);
}

#[test]
fn default_not_paused() {
    let tick = TickResource::default();
    assert!(!tick.paused);
}

#[test]
fn current_day_0_before_first_day() {
    let tick = TickResource {
        tick_count: 0,
        ..Default::default()
    };
    assert_eq!(tick.current_day(), 0);
}

#[test]
fn current_day_0_at_59999() {
    let tick = TickResource {
        tick_count: 59999,
        ..Default::default()
    };
    assert_eq!(tick.current_day(), 0);
}

#[test]
fn current_day_1_at_60000() {
    let tick = TickResource {
        tick_count: 60000,
        ..Default::default()
    };
    assert_eq!(tick.current_day(), 1);
}

#[test]
fn current_day_2_at_120000() {
    let tick = TickResource {
        tick_count: 120000,
        ..Default::default()
    };
    assert_eq!(tick.current_day(), 2);
}

#[test]
fn tick_of_day_0_at_start() {
    let tick = TickResource {
        tick_count: 0,
        ..Default::default()
    };
    assert_eq!(tick.tick_of_day(), 0);
}

#[test]
fn tick_of_day_1_after_first_tick() {
    let tick = TickResource {
        tick_count: 1,
        ..Default::default()
    };
    assert_eq!(tick.tick_of_day(), 1);
}

#[test]
fn tick_of_day_resets_on_new_day() {
    let tick = TickResource {
        tick_count: 60000,
        ..Default::default()
    };
    assert_eq!(tick.tick_of_day(), 0);
}

#[test]
fn tick_of_day_1_into_second_day() {
    let tick = TickResource {
        tick_count: 60001,
        ..Default::default()
    };
    assert_eq!(tick.tick_of_day(), 1);
}

#[test]
fn day_progress_0_at_midnight() {
    let tick = TickResource {
        tick_count: 0,
        ..Default::default()
    };
    assert!((tick.day_progress() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn day_progress_0_5_at_midday() {
    let tick = TickResource {
        tick_count: 30000,
        ..Default::default()
    };
    assert!((tick.day_progress() - 0.5).abs() < 1e-10);
}

#[test]
fn tick_duration_constant() {
    let expected = 1000.0 / 60000.0;
    assert!((TickResource::TICK_DURATION - expected).abs() < f64::EPSILON);
}
