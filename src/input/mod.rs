pub mod camera;

use crate::core::state::GameState;
use bevy::prelude::*;
use camera::camera_control;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_control.run_if(in_state(GameState::Playing)));
    }
}

// TODO: Input de gameplay (próxima etapa)
// =======================================
// - src/input/mouse.rs:
//   • WorldUnderCursor: recurso atualizado a cada frame com a posição
//     do mouse no mundo (raycast 2D do tile abaixo do cursor).
//   • Eventos de clique: MouseButtonInput → emite eventos como
//     PlaceBlueprint, SelectEntity, CancelAction.
//
// - src/input/commands.rs:
//   • Teclas de atalho (B = blueprint, ESC = cancelar, etc.)
//     → emitem eventos de intenção. NÃO alteram simulação diretamente.
//
// Regra:
//   Input de gameplay JAMAIS mexe em estado de simulação.
//   Cria eventos. Sistemas de validação na simulação escutam, validam,
//   e só então aplicam a mudança.
