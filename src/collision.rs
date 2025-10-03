use crate::enemy::Enemy;
use crate::gun::Bullet;
use crate::state::GameState;
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_enemy_bullet_collision.run_if(in_state(GameState::GameInit)),
        );
    }
}

fn handle_enemy_bullet_collision(
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
    bullet_query: Query<&Transform, (With<Bullet>, Without<Enemy>)>,
) {
    bullet_query.iter().for_each(|bullet_transform| {
        enemy_query
            .iter_mut()
            .for_each(|(enemy_transform, mut enemy)| {
                if bullet_transform
                    .translation
                    .distance_squared(enemy_transform.translation)
                    <= 1000.
                {
                    enemy.health -= 1;
                }
            })
    })
}
