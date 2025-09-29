use crate::game::window::{Star, StarDrift};
use crate::game::GameScene;
use crate::ui_component::UiTheme;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use rand::Rng;

pub(super) fn setup_game_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut clear_color: ResMut<ClearColor>,
    mut query: Query<Entity, With<Star>>,
) {
    clear_color.0 = Color::BLACK;

    if !query.is_empty() {
        info!("星空背景已经生成：复用");
        for entity in &mut query {
            commands.entity(entity).insert(Visibility::Visible);
        }
        return;
    }

    let num_stars = 100;
    let mut rng = rand::thread_rng();

    for _ in 0..num_stars {
        let x = rng.gen_range(-75.0..75.0);
        let y = rng.gen_range(-10.0..0.0);
        let z = rng.gen_range(-75.0..75.0);
        let size = rng.gen_range(0.01..0.15);

        // 星星随机缓慢漂浮速度
        let drift = Vec2::new(rng.gen_range(-0.2..0.2), rng.gen_range(-0.2..0.2));

        commands.spawn((
            Mesh3d(meshes.add(Circle::new(size))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::hsl(0.0, 0.0, rng.gen_range(0.7..1.0)),
                emissive: LinearRgba::WHITE,
                ..Default::default()
            })),
            Transform::from_xyz(x, y, z).with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            Star,
            StarDrift { velocity: drift },
            Bloom::NATURAL,
        ));
    }

    info!("生成星空背景成功");
}

/// 星星无限平铺
pub(super) fn wrap_stars_around_camera(
    camera_query: Query<&Transform, (With<Camera3d>, Without<Star>)>,
    mut star_query: Query<&mut Transform, With<Star>>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let cam_x = camera_transform.translation.x;
    let cam_z = camera_transform.translation.z;
    let bounds = 70.0;

    for mut transform in &mut star_query {
        let mut pos = transform.translation;

        // X wrap
        if pos.x < cam_x - bounds {
            pos.x += bounds * 2.0;
        } else if pos.x > cam_x + bounds {
            pos.x -= bounds * 2.0;
        }

        // Y wrap
        if pos.z < cam_z - bounds {
            pos.z += bounds * 2.0;
        } else if pos.z > cam_z + bounds {
            pos.z -= bounds * 2.0;
        }

        transform.translation = pos;
    }
}

/// 星星缓慢漂浮
pub(super) fn drift_stars(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &StarDrift), With<Star>>,
) {
    let delta = time.delta_secs();

    for (mut transform, drift) in &mut query {
        transform.translation.x += drift.velocity.x * delta;
        transform.translation.y += drift.velocity.y * delta;
    }
}

pub(super) fn restore_menu_background(mut clear_color: ResMut<ClearColor>, ui_theme: Res<UiTheme>) {
    // 退出游戏状态时恢复菜单的背景色
    clear_color.0 = ui_theme.bg_color;
}

pub(super) fn spawn_game_main_scene(
    mut commands: Commands,
    mut query: Query<Entity, With<GameScene>>,
) {
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

pub(super) fn hide_spawn_game_main_scene(
    mut commands: Commands,
    mut query: Query<Entity, With<GameScene>>,
) {
    if let Ok(entity) = query.single_mut() {
        commands.entity(entity).insert(Visibility::Hidden);
        return;
    }
}
