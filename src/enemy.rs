use crate::animation::AnimationTimer;
use crate::player::Player;
use crate::state::GameState;
use crate::{
    GlobalTextureAtlas, ENEMY_BASE_INDEX, ENEMY_SPAWN_INTERVAL, ENEMY_SPEED, MAX_NUM_ENEMIES,
    SPRITE_SCALE_FACTOR, WORLD_HEIGHT, WORLD_WIDTH,
};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::prelude::IndexedRandom;
use rand::prelude::IteratorRandom;
use rand::Rng;
use std::time::Duration;

pub struct EnemyPlugin;

#[derive(Component, Default)]
pub struct Enemy {
    pub index: usize,
    pub health: u32,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                update_enemy_transform,
            )
                .run_if(in_state(GameState::GameInit)),
        );
    }
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
        let x = rng.random_range(-WORLD_WIDTH..WORLD_WIDTH);
        let y = rng.random_range(-WORLD_HEIGHT..WORLD_HEIGHT);
        let index = *ENEMY_BASE_INDEX.choose(&mut rng).unwrap();

        commands.spawn((
            Enemy { index, ..default() },
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
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
}
