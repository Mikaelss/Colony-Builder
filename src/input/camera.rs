// NOTA: Camera mexe Transform diretamente porque é puramente
// apresentacional. Input de gameplay (mouse, teclas de ação)
// deve usar eventos — veja mod.rs para a estratégia.

use crate::presentation::sprites::TILE_SIZE;
use crate::world::{GRID_HEIGHT, GRID_WIDTH};
use bevy::prelude::*;

const SPEED: f32 = 600.0;

pub fn camera_control(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut transform = match query.single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

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
        delta = delta.normalize() * SPEED * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }

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
