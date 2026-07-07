use crate::core::event::TickEvent;
use crate::core::time::TickResource;
use crate::debug::DebugSettings;
use crate::presentation::sprites::TILE_SIZE;
use bevy::prelude::*;

pub fn tick_log(settings: Res<DebugSettings>, mut reader: MessageReader<TickEvent>) {
    if !settings.dev_mode || !settings.show_tick {
        return;
    }
    for ev in reader.read() {
        println!(
            "[Tick] Dia {}, tick {}/{} ({:.1}%)",
            ev.day + 1,
            ev.tick_of_day,
            TickResource::TICKS_PER_DAY,
            ev.tick_of_day as f64 / TickResource::TICKS_PER_DAY as f64 * 100.0,
        );
    }
}

pub fn render_metrics(
    settings: Res<DebugSettings>,
    camera_query: Query<&Projection, With<Camera2d>>,
    window_query: Query<&Window>,
    time: Res<Time>,
    mut frame_counter: Local<u32>,
) {
    if !settings.dev_mode || !settings.show_metrics {
        return;
    }
    *frame_counter += 1;
    if !(*frame_counter).is_multiple_of(60) {
        return;
    }

    let scale = match camera_query.single() {
        Ok(projection) => match projection {
            Projection::Orthographic(ortho) => ortho.scale,
            _ => return,
        },
        Err(_) => return,
    };

    let window = match window_query.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let tiles_x = window.width() * scale / TILE_SIZE;
    let tiles_y = window.height() * scale / TILE_SIZE;
    let fps = 1.0 / time.delta_secs();

    println!(
        "[Render] zoom: {scale:.2} | tiles visiveis: {:.0} | FPS: {fps:.0}",
        tiles_x * tiles_y,
    );
}
