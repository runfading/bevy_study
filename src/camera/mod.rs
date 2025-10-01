use bevy::app::App;
use bevy::camera::Camera2d;
use bevy::math::Vec3;
use bevy::prelude::{default, Commands, DirectionalLight, Plugin, Startup, Transform};

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                Camera2d,
                // Transform::from_translation(Vec3::new(0.0, 50.0, 0.0))
                //     .looking_at(Vec3::ZERO, Vec3::Z),
            ));
            commands.spawn(DirectionalLight {
                illuminance: 400.0,
                ..default()
            });
        });
    }
}
