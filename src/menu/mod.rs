use crate::menu::menu::spawn_main_menu;
use crate::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;

pub mod menu;

// 菜单插件
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), menu::despawn_main_menu)
            .add_systems(
                Update,
                menu::handle_esc_key.run_if(in_state(GameState::InGame)),
            ); // 添加退出时的清理
    }
}
