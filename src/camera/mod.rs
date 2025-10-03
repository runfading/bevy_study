use bevy::app::App;
use bevy::camera::Camera2d;
use bevy::prelude::{Commands, Msaa, Plugin, Startup, Transform, Vec3};

pub(super) mod pixel;

// 定义缩放因子（比如 2x, 3x）
pub(self) const SCALE: f32 = 3.0;

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                Camera2d::default(),
                Transform::from_scale(Vec3::splat(SCALE)),
                Msaa::Off
            ));
        })
        // .add_systems(Update, snap_to_pixel_grid)

        ;
    }
}
