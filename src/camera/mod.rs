use bevy::app::App;
use bevy::math::Vec3;
use bevy::pbr::PointLight;
use bevy::prelude::{default, Camera3d, Commands, Plugin, Startup, Transform};

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                Camera3d::default(),
                Transform::from_translation(Vec3::new(0.0, 0.0, 100.0))
                    .looking_at(Vec3::ZERO, Vec3::Y),
            ));
            commands.spawn((
                PointLight {
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ));
        });
    }
}
