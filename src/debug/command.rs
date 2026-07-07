use crate::core::definitions::item::ItemRegistry;
use crate::core::save::SaveCommand;
use crate::debug::DebugSettings;
use bevy::prelude::*;

#[derive(Message, Debug)]
pub enum DebugCommand {
    SpawnItem(String),
}

pub fn handle_debug_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<DebugSettings>,
    registry: Option<Res<ItemRegistry>>,
    mut cmd_writer: MessageWriter<DebugCommand>,
    mut save_writer: MessageWriter<SaveCommand>,
    mut current_spawn: Local<usize>,
) {
    if keys.just_pressed(KeyCode::F3) {
        settings.dev_mode = !settings.dev_mode;
        println!(
            "[Debug] Dev mode {}",
            if settings.dev_mode { "ON" } else { "OFF" }
        );
        return;
    }

    if !settings.dev_mode {
        return;
    }

    if keys.just_pressed(KeyCode::F1) {
        settings.show_tick = !settings.show_tick;
        println!(
            "[Debug] Tick log {}",
            if settings.show_tick { "ON" } else { "OFF" }
        );
        return;
    }

    if keys.just_pressed(KeyCode::F2) {
        settings.show_metrics = !settings.show_metrics;
        println!(
            "[Debug] Metrics log {}",
            if settings.show_metrics { "ON" } else { "OFF" }
        );
        return;
    }

    if keys.just_pressed(KeyCode::F5) {
        save_writer.write(SaveCommand::Save("save_0".into()));
        println!("[Debug] Save triggered");
        return;
    }

    if keys.just_pressed(KeyCode::F9) {
        save_writer.write(SaveCommand::Load("save_0".into()));
        println!("[Debug] Load triggered");
        return;
    }

    if keys.just_pressed(KeyCode::KeyG) {
        let registry = match registry {
            Some(ref r) => r,
            None => return,
        };
        let items: Vec<_> = registry.all().collect();
        if items.is_empty() {
            return;
        }
        *current_spawn = (*current_spawn + 1) % items.len();
        let def = items[*current_spawn];
        cmd_writer.write(DebugCommand::SpawnItem(def.id.clone()));
        if settings.show_spawns {
            println!("[Debug] Spawning: {}", def.name);
        }
    }
}
