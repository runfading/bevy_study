use crate::menu::menu::MenuPlugin;
use crate::ui_component::{ButtonPlugins, UiTheme};
use bevy::app::Startup;
use bevy::prelude::*;
use bevy::DefaultPlugins;

mod menu;
mod ui_component;

fn main() {
    let ui_theme = UiTheme::default();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ui_theme.clone())
        .init_state::<GameState>()
        .insert_resource(ClearColor(ui_theme.bg_color))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .add_plugins(ButtonPlugins)
        .add_plugins(MenuPlugin)
        .run();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}
