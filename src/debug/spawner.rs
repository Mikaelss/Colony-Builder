use crate::core::definitions::item::ItemRegistry;
use crate::core::identity::{EntityMap, IdAllocator, IdComponent};
use crate::debug::{DebugCommand, DebugItem, DebugSettings};
use bevy::prelude::*;

pub fn handle_spawn_command(
    settings: Res<DebugSettings>,
    registry: Res<ItemRegistry>,
    mut allocator: ResMut<IdAllocator<DebugItem>>,
    mut entity_map: ResMut<EntityMap<DebugItem>>,
    mut reader: MessageReader<DebugCommand>,
    mut commands: Commands,
) {
    if !settings.dev_mode {
        return;
    }

    for cmd in reader.read() {
        match cmd {
            DebugCommand::SpawnItem(item_id) => {
                let def = match registry.get(item_id) {
                    Some(d) => d,
                    None => {
                        if settings.show_spawns {
                            println!("[Debug] Unknown item: {item_id}");
                        }
                        continue;
                    }
                };

                let id = allocator.next();
                let entity = commands
                    .spawn((
                        IdComponent::<DebugItem> { id },
                        Name::new(format!("Debug: {}", def.name)),
                    ))
                    .id();

                entity_map.insert(id, entity);

                if settings.show_spawns {
                    println!(
                        "[Debug] Spawned {} (Id: {}, Entity: {:?})",
                        def.name,
                        id.get(),
                        entity
                    );
                }
            }
        }
    }
}
