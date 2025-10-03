use crate::assets::AssetLoaderPlugin;
use crate::camera::CameraPlugin;
use crate::game::GamePlugin;
use crate::menu::MenuPlugin;
use crate::ui_component::{ButtonPlugins, UiTheme};
use bevy::prelude::*;
use bevy::DefaultPlugins;

mod assets;
mod camera;
mod game;
mod menu;
mod ui_component;

fn main() {
    let ui_theme = UiTheme::default();

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ui_theme.clone())
        .init_state::<GameState>()
        .insert_resource(ClearColor(ui_theme.bg_color))
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(OnEnter(GameState::InGame), || info!("进入游戏状态"))
        .add_plugins(ButtonPlugins)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .run();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    MainMenu,
    InGame,
    Paused,
}
