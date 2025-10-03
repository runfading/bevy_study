use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use test_bevy_ui::camera::FollowCameraPlugin;
use test_bevy_ui::gun::GunPlugin;
use test_bevy_ui::player::PlayerPlugin;
use test_bevy_ui::state::GameState;
use test_bevy_ui::world::WorldPlugin;
use test_bevy_ui::{ResourcesPlugin, BG_COLOR, WINDOW_HEIGHT, WINDOW_WIDTH};
use test_bevy_ui::collision::CollisionPlugin;
use test_bevy_ui::enemy::EnemyPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(FollowCameraPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(test_bevy_ui::animation::AnimationPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}
