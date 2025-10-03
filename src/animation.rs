use crate::enemy::Enemy;
use crate::gun::Gun;
use crate::player::{Player, PlayerState};
use crate::state::GameState;
use crate::CursorPosition;
use bevy::prelude::*;

pub struct AnimationPlugin;

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animate_time_tick,
                animate_player,
                animate_enemy,
                flip_gun_sprite_y,
                flip_player_sprite_x,
                flip_enemy_sprite_x,
            )
                .run_if(in_state(GameState::GameInit)),
        );
    }
}

/// 触发定时器
fn animate_time_tick(mut timer_query: Query<&mut AnimationTimer>, time: Res<Time>) {
    timer_query.iter_mut().for_each(|mut timer| {
        timer.0.tick(time.delta());
    })
}

fn animate_player(
    mut player_query: Query<(&mut Sprite, &PlayerState, &AnimationTimer), With<Player>>,
) {
    player_query
        .iter_mut()
        .for_each(|(mut sprite, play_state, timer)| {
            if timer.0.just_finished() {
                if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
                    let base_sprite_index = match play_state {
                        PlayerState::Idle => 0,
                        PlayerState::Moving => 4,
                    };
                    texture_atlas.index = base_sprite_index + (texture_atlas.index + 1) % 4;
                }
            }
        });
}

fn animate_enemy(mut enemy_query: Query<(&Enemy, &mut Sprite, &AnimationTimer), With<Enemy>>) {
    enemy_query
        .iter_mut()
        .for_each(|(enemy, mut sprite, timer)| {
            if timer.0.just_finished() {
                if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
                    texture_atlas.index = enemy.0 + (texture_atlas.index + 1) % 4;
                }
            }
        });
}

fn flip_enemy_sprite_x(
    mut enemy_query: Query<(&mut Sprite, &Transform), With<Enemy>>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    let player_pos = if let Ok(player_transform) = player_query.single() {
        player_transform.translation
    } else {
        return;
    };

    enemy_query.iter_mut().for_each(|(mut sprite, transform)| {
        if player_pos.x > transform.translation.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    });
}

fn flip_player_sprite_x(
    mut player_query: Query<(&mut Sprite, &Transform), With<Player>>,
    cursor_position: Res<CursorPosition>,
) {
    player_query.iter_mut().for_each(|(mut sprite, transform)| {
        if let Some(cursor_pos) = cursor_position.0 {
            if cursor_pos.x > transform.translation.x {
                sprite.flip_x = false;
            } else {
                sprite.flip_x = true;
            }
        }
    });
}

fn flip_gun_sprite_y(
    mut gun_query: Query<(&mut Sprite, &Transform), With<Gun>>,
    cursor_position: Res<CursorPosition>,
) {
    gun_query.iter_mut().for_each(|(mut sprite, transform)| {
        if let Some(cursor_pos) = cursor_position.0 {
            if cursor_pos.x > transform.translation.x {
                sprite.flip_y = false;
            } else {
                sprite.flip_y = true;
            }
        }
    });
}
