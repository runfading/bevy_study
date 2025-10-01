use crate::game::entity_lifecycle::Mario;
use bevy::prelude::{Commands, Entity, Query, Visibility, With};

/// 隐藏实体
pub(super) fn despawn_ball(mut commands: Commands, mut query: Query<Entity, With<Mario>>) {
    if let Ok(ball) = query.single_mut() {
        commands.entity(ball).insert(Visibility::Hidden);
    }
}
