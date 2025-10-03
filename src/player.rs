use crate::collision::EnemyKdTree;
use crate::configs::PLAYER_SPEED;
use crate::state::GameState;
use crate::ENEMY_DAMAGE;
use bevy::app::App;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayHealth(pub f32);

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Moving,
}

#[derive(Event)]
pub struct LifeReduce;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LifeReduce>().add_systems(
            Update,
            (
                handle_player_input,
                handle_enemy_collision,
                handle_player_death,
                handle_player_health_event,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_death(
    play_query: Query<&mut PlayHealth, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(health) = play_query.single() {
        if health.0 <= 0. {
            game_state.set(GameState::MainMenu)
        }
    }
}

fn handle_player_health_event(
    mut play_query: Query<&mut PlayHealth, With<Player>>,
    mut event_reader: EventReader<LifeReduce>,
) {
    if let Ok(mut health) = play_query.single_mut() {
        for _ in event_reader.read() {
            health.0 -= ENEMY_DAMAGE;
        }
    }
}

fn handle_enemy_collision(
    mut play_query: Query<(&mut PlayHealth, &Transform), With<Player>>,
    tree: Res<EnemyKdTree>,
) {
    if let Ok((mut health, transform)) = play_query.single_mut() {
        let player_pos = transform.translation;
        let enemies = tree.0.within_radius(&[player_pos.x, player_pos.y], 50.);

        enemies.iter().for_each(|_| {
            health.0 -= 1.0;
        })
    }
}

fn handle_player_input(
    mut player: Query<(&mut Transform, &mut PlayerState), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut transform, mut player_state) = if let Ok(transform) = player.single_mut() {
        transform
    } else {
        return;
    };

    let mut delta = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        delta.y += 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        delta.y -= 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        delta.x -= 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        delta.x += 1.;
    }

    delta = delta.normalize();

    // 向量是否是有限的
    if delta.is_finite() {
        transform.translation += vec3(delta.x, delta.y, 0.) * PLAYER_SPEED;
        *player_state = PlayerState::Moving;
    } else {
        *player_state = PlayerState::Idle;
    }
}
