use crate::game::window::Star;
use crate::game::GameScene;
use crate::ui_component::UiTheme;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::prelude::*;
use rand::Rng;

pub(super) fn setup_game_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut clear_color: ResMut<ClearColor>,
    _theme: Res<UiTheme>,
    mut query: Query<Entity, With<Star>>,
) {
    // 设置游戏状态下的窗口背景色为黑色
    clear_color.0 = Color::BLACK;

    if !query.is_empty() {
        info!("星空背景已经生成：复用");
        for entity in &mut query {
            commands.entity(entity).insert(Visibility::Visible);
        }
        return;
    }

    let num_stars = 500; // 星星数量

    let mut rng = rand::thread_rng();

    for _ in 0..num_stars {
        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);
        let z = rng.gen_range(-10.0..0.); // 3D 深度感
        let size = rng.gen_range(0.01..0.15);

        commands.spawn((
            Mesh3d(meshes.add(Circle::new(size))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::hsl(0.0, 0.0, rng.gen_range(0.7..1.0)),
                emissive: LinearRgba::WHITE,
                ..Default::default()
            })),
            Transform::from_xyz(x, y, z),
            Star,
        ));
    }

    info!("生成星空背景成功");
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
