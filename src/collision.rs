use crate::enemy::Enemy;
use crate::gun::Bullet;
use crate::state::GameState;
use crate::KD_TREE_REFRESH_RATE;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use kd_tree::{KdPoint, KdTree};
use std::time::Duration;

pub struct CollisionPlugin;

pub struct Collisible {
    pos: Vec2,
    entity: Entity,
}

impl KdPoint for Collisible {
    type Scalar = f32;
    type Dim = typenum::U2;

    fn at(&self, k: usize) -> f32 {
        if k == 0 {
            return self.pos.x;
        }

        self.pos.y
    }
}

#[derive(Resource)]
pub struct EnemyKdTree(pub(crate) KdTree<Collisible>);

impl Default for EnemyKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyKdTree>().add_systems(
            Update,
            (
                handle_enemy_bullet_collision,
                update_enemy_kd_tree
                    .run_if(on_timer(Duration::from_secs_f32(KD_TREE_REFRESH_RATE))),
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_enemy_kd_tree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    let mut items = Vec::new();

    for (enemy, transform) in enemy_query.iter() {
        items.push(Collisible {
            pos: transform.translation.truncate(),
            entity: enemy,
        })
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

fn handle_enemy_bullet_collision(
    mut enemy_query: Query<&mut Enemy, With<Enemy>>,
    bullet_query: Query<&Transform, (With<Bullet>, Without<Enemy>)>,
    tree: Res<EnemyKdTree>,
) {
    bullet_query.iter().for_each(|bullet_transform| {
        let bullet_pos = bullet_transform.translation;

        let enemies = tree.0.within_radius(&[bullet_pos.x, bullet_pos.y], 50.);

        // 一个子弹只能处理一个敌人
        for coll in enemies {
            if let Ok(mut enemy) = enemy_query.get_mut(coll.entity) {
                enemy.health -= 1;
                break;
            }
        }
    })
}
