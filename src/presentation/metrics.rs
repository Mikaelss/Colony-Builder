use crate::presentation::sprites::TILE_SIZE;
use bevy::prelude::*;

pub fn log_render_metrics(
    camera_query: Query<&Projection, With<Camera2d>>,
    window_query: Query<&Window>,
    time: Res<Time>,
    mut frame_counter: Local<u32>,
) {
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
