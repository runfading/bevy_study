mod action;
mod entity_lifecycle;
mod window;

use crate::game::action::MovementPlugin;
use crate::game::entity_lifecycle::LifecyclePlugin;
use crate::game::window::WindowPlugin;
use bevy::app::App;
use bevy::prelude::{Component, Plugin};

#[derive(Component)]
pub struct GameScene;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowPlugin)
            .add_plugins(LifecyclePlugin)
            .add_plugins(MovementPlugin);
    }
}
