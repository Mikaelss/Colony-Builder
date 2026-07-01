// NOTA: Camera mexe Transform e Projection diretamente porque é
// puramente apresentacional. Input de gameplay (mouse, teclas de
// ação) deve usar eventos — veja mod.rs para a estratégia.

use crate::presentation::sprites::TILE_SIZE;
use crate::world::{GRID_HEIGHT, GRID_WIDTH};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

const PAN_SPEED: f32 = 600.0;
const ZOOM_FACTOR: f32 = 1.15;
const MIN_SCALE: f32 = 0.1;
const MAX_SCALE: f32 = 20.0;

pub fn camera_control(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut scroll_events: MessageReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
) {
    let (mut transform, mut projection) = match query.single_mut() {
        Ok(pair) => pair,
        Err(_) => return,
    };

    let ortho = match projection.as_mut() {
        Projection::Orthographic(ortho) => ortho,
        _ => return,
    };
    let scale = ortho.scale;

    let mut delta = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        delta.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        delta.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        delta.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        delta.x += 1.0;
    }

    if delta != Vec2::ZERO {
        delta = delta.normalize() * PAN_SPEED * time.delta_secs() * scale;
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }

    let mut new_scale = scale;
    for event in scroll_events.read() {
        if event.y > 0.0 {
            new_scale /= ZOOM_FACTOR;
        } else if event.y < 0.0 {
            new_scale *= ZOOM_FACTOR;
        }
    }
    if keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd) {
        new_scale /= ZOOM_FACTOR;
    }
    if keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract) {
        new_scale *= ZOOM_FACTOR;
    }
    ortho.scale = new_scale.clamp(MIN_SCALE, MAX_SCALE);

    if keyboard.just_pressed(KeyCode::KeyH) {
        let center_x = GRID_WIDTH as f32 * TILE_SIZE / 2.0;
        let center_y = GRID_HEIGHT as f32 * TILE_SIZE / 2.0;
        transform.translation.x = center_x;
        transform.translation.y = center_y;
    }

    let map_w = GRID_WIDTH as f32 * TILE_SIZE;
    let map_h = GRID_HEIGHT as f32 * TILE_SIZE;
    transform.translation.x = transform.translation.x.clamp(0.0, map_w);
    transform.translation.y = transform.translation.y.clamp(0.0, map_h);
}
