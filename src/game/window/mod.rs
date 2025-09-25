use crate::game::GameScene;
use crate::ui_component::UiTheme;
use crate::GameState;
use bevy::app::App;
use bevy::color::Color;
use bevy::prelude::*;

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

fn setup_game_background(mut clear_color: ResMut<ClearColor>, _theme: Res<UiTheme>) {
    // 设置游戏状态下的窗口背景色为黑色
    clear_color.0 = Color::BLACK;
}

fn restore_menu_background(mut clear_color: ResMut<ClearColor>, ui_theme: Res<UiTheme>) {
    // 退出游戏状态时恢复菜单的背景色
    clear_color.0 = ui_theme.bg_color;
}

fn spawn_game_main_scene(mut commands: Commands, mut query: Query<Entity, With<GameScene>>) {
    if let Ok(entity) = query.single_mut() {
        commands.entity(entity).insert(Visibility::Visible);
        return;
    }

    commands.spawn((
        Node {
            width: Val::Auto,  // 占窗口宽度80%
            height: Val::Auto, // 占窗口高度80%
            ..default()
        },
        // 透明色
        BackgroundColor(Color::NONE),
        GameScene,
    ));
}

fn hide_spawn_game_main_scene(mut commands: Commands, mut query: Query<Entity, With<GameScene>>) {
    if let Ok(entity) = query.single_mut() {
        commands.entity(entity).insert(Visibility::Hidden);
        return;
    }
}
