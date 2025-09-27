use crate::game::entity_lifecycle::Ball;
use bevy::prelude::*;

pub(super) fn look(
    mut commands: Commands,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<Ball>)>,
    mut ball_transforms: Query<&mut GlobalTransform, With<Ball>>,
) {


}
