mod bg;

use crate::game::window::bg::*;
use crate::GameState;
use bevy::app::App;
use bevy::prelude::*;

#[derive(Component)]
pub(self) struct Star;

pub(super) struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (setup_game_background, spawn_game_main_scene),
        )
        .add_systems(
            OnExit(GameState::InGame),
            (restore_menu_background, hide_spawn_game_main_scene),
        );
    }
}
