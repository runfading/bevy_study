use bevy::prelude::{Commands, Entity, Query, Visibility, With};
use crate::game::entity_lifecycle::Ball;

/// 隐藏实体
pub(super) fn despawn_ball(mut commands: Commands, mut query: Query<Entity, With<Ball>>) {
    if let Ok(ball) = query.single_mut() {
        commands.entity(ball).insert(Visibility::Hidden);
    }
}
