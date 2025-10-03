use crate::enemy::Enemy;
use crate::player::{PlayHealth, Player};
use crate::state::GameState;
use crate::world::GameEntity;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct GuiPlugin;

#[derive(Component)]
struct DebugText;

#[derive(Component)]
struct MainMenu;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInit), spawn_debug_text)
            .add_systems(
                Update,
                update_debug_text.run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), hide_main_menu)
            .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)));
    }
}

fn hide_main_menu(mut commands: Commands, mut menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu) = menu_query.single_mut() {
        commands.entity(main_menu).insert(Visibility::Hidden);
    }
}

fn button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => game_state.set(GameState::GameInit),
            _ => {}
        }
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu_query: Query<Entity, With<MainMenu>>,
) {
    // 有主菜单不重复生成
    if let Ok(main_menu) = menu_query.single_mut() {
        commands.entity(main_menu).insert(Visibility::Visible);
        return;
    }

    commands.spawn((
        MainMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        children![button(&asset_server)],
    ));
}

fn button(asset_server: &AssetServer) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(Color::WHITE),
            children![(
                Text::new("Play"),
                TextFont {
                    font: asset_server.load("monogram.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::BLACK),
                // TextShadow::default(),
            )]
        )],
    )
}

fn spawn_debug_text(mut commands: Commands) {
    commands.spawn((
        DebugText,
        Text::new(""),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font_size: 42.0,
            ..default()
        },
        GameEntity,
    ));
}

fn update_debug_text(
    mut debug_text_query: Query<&mut Text, With<DebugText>>,
    enemy_query: Query<(), With<Enemy>>,
    player_query: Query<&PlayHealth, With<Player>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let mut text = if let Ok(text) = debug_text_query.single_mut() {
        text
    } else {
        return;
    };
    let player_health = if let Ok(player_health) = player_query.single() {
        player_health.0
    } else {
        return;
    };
    let enemy_count = enemy_query.iter().count();

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = fps.smoothed() {
            text.0 = format!(
                "FPS: {:.2}\nEnemy Num:{}\nHealth:{}",
                fps, enemy_count, player_health
            );
        }
    }
}
