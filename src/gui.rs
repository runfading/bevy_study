use crate::enemy::Enemy;
use crate::state::GameState;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct GuiPlugin;

#[derive(Component)]
struct DebugText;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_debug_text).add_systems(
            Update,
            update_debug_text.run_if(in_state(GameState::GameInit)),
        );
    }
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
    ));
}

fn update_debug_text(
    mut debug_text_query: Query<&mut Text, With<DebugText>>,
    enemy_query: Query<(), With<Enemy>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let mut text = if let Ok(text) = debug_text_query.single_mut() {
        text
    } else {
        return;
    };
    let enemy_count = enemy_query.iter().count();

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = fps.smoothed() {
            text.0 = format!("FPS: {:.2}\nEnemy Num:{}", fps, enemy_count);
        }
    }
}
