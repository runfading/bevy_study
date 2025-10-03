use crate::animation::AnimationTimer;
use crate::player::Player;
use crate::state::GameState;
use crate::{
    GlobalTextureAtlas, ENEMY_BASE_INDEX, ENEMY_SPAWN_INTERVAL, ENEMY_SPEED, MAX_NUM_ENEMIES,
    SPRITE_SCALE_FACTOR,
};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::prelude::IndexedRandom;
use rand::prelude::IteratorRandom;
use rand::Rng;
use std::f32::consts::PI;
use std::time::Duration;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub index: usize,
    pub health: i32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            index: 0,
            health: 2,
        }
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                update_enemy_transform,
                despawn_enemy,
            )
                .run_if(in_state(GameState::GameInit)),
        );
    }
}

fn despawn_enemy(mut commands: Commands, enemy_query: Query<(Entity, &Enemy), With<Enemy>>) {
    enemy_query.iter().for_each(|(entity, enemy)| {
        if enemy.health <= 0 {
            commands.entity(entity).despawn();
        }
    });
}

fn update_enemy_transform(
    player_query: Query<&Transform, (With<Player>)>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    let player_transform = if let Ok(transform) = player_query.single() {
        transform
    } else {
        return;
    };

    enemy_query.iter_mut().for_each(|mut enemy_transform| {
        // 敌人指向玩家的向量，两个坐标反着做减法，就能得到方向向量
        let dir = (player_transform.translation - enemy_transform.translation).normalize();

        enemy_transform.translation += dir * ENEMY_SPEED;
    });
}

fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(10);

    if num_enemies >= MAX_NUM_ENEMIES {
        return;
    }

    let player_pos = if let Ok(player_transform) = player_query.single() {
        player_transform.translation.truncate()
    } else {
        return;
    };

    let mut rng = rand::rng();

    for _ in 0..enemy_spawn_count {
        let (x, y) = get_random_position_around(player_pos);
        let index = *ENEMY_BASE_INDEX.choose(&mut rng).unwrap();

        commands.spawn((
            Enemy { index, ..default() },
            AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
            Transform::from_translation(vec3(x, y, 1.))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Sprite::from_atlas_image(
                handle.image.clone().unwrap(),
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index,
                },
            ),
        ));
    }
}

/// 获取随机位置
fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::rng();
    // 0..2PI，即0~360°
    let angle = rng.random_range(0.0..PI * 2.0);
    // 1000~5000距离
    let dist = rng.random_range(1000.0..5000.0);

    // 公式 cos弧度*dis x坐标偏移量
    let offset_x = angle.cos() * dist;
    // 公式 sin弧度*dis y坐标偏移量
    let offset_y = angle.sin() * dist;

    // 原点坐标+偏移量=随机坐标
    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let mut rng = rand::rng();
        for _ in 0..100 {
            let x = ENEMY_BASE_INDEX.choose(&mut rng).unwrap();
            println!("{:?}", x)
        }
    }

    #[test]
    fn test_usize() {
        // let num: u32 = 1;
        // 整型溢出
        // println!("{}", num - 100);
    }
}
