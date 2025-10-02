use crate::configs::{BULLET_SPAWN_INTERVAL, BULLET_SPEED, SPRITE_SCALE_FACTOR};
use crate::player::Player;
use crate::resources::{CursorPosition, GlobalTextureAtlas};
use crate::state::GameState;
use bevy::prelude::*;
use bevy::time::Stopwatch;

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;
#[derive(Component)]
pub struct GunTimer(pub Stopwatch);
#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
struct BulletDirection(Dir3);

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_gun_input, update_gun_transform, update_bullets)
                .run_if(in_state(GameState::GameInit)),
        );
    }
}

/// 鼠标左键发射子弹
fn handle_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun: Query<(&Transform, &mut GunTimer), With<Gun>>,
    handle: Res<GlobalTextureAtlas>,
    keyboard_input: Res<ButtonInput<MouseButton>>,
) {
    if !keyboard_input.pressed(MouseButton::Left) {
        return;
    }

    let (gun_pos, gun_transform, mut gun_timer) =
        if let Ok((transform, gun_timer)) = gun.single_mut() {
            (transform.translation.truncate(), transform, gun_timer)
        } else {
            return;
        };

    gun_timer.0.tick(time.delta());
    if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        gun_timer.0.reset();

        commands.spawn((
            Bullet,
            BulletDirection(gun_transform.local_x()),
            Transform::from_translation(vec3(gun_pos.x, gun_pos.y, 0.))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Sprite::from_atlas_image(
                handle.image.clone().unwrap(),
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 16,
                },
            ),
        ));
    }
}

fn update_gun_transform(
    cursor_position: Res<CursorPosition>,
    mut gun: Query<&mut Transform, With<Gun>>,
    player: Query<&Transform, (With<Player>, Without<Gun>)>,
) {
    let play_pos = if let Ok(transform) = player.single() {
        // 放弃z轴向量
        transform.translation.truncate()
    } else {
        return;
    };

    let mut gun_transform = if let Ok(transform) = gun.single_mut() {
        transform
    } else {
        return;
    };

    let cursor_pos = if let Some(cursor_pos) = cursor_position.0 {
        cursor_pos
    } else {
        play_pos
    };

    // 玩家指向鼠标的向量
    let to_cursor = cursor_pos - play_pos;
    // 玩家指向鼠标
    let angle = to_cursor.y.atan2(to_cursor.x);

    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.;
    let new_gun_pos = vec2(
        play_pos.x + offset * angle.cos(),
        play_pos.y + offset * angle.sin(),
    );

    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
    log::info!("{:?}", cursor_position.0);
}

fn update_bullets(mut bullet_direction: Query<(&mut Transform, &BulletDirection), With<Bullet>>) {
    for (mut transform, direction) in bullet_direction.iter_mut() {
        // 使translation 沿 direction方向移动
        transform.translation += direction.0.normalize() * Vec3::splat(BULLET_SPEED);
        transform.translation.z = 10.;
    }
}
