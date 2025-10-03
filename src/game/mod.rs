mod action;
mod animation;
mod camera;
mod config;
mod player;

use crate::game::action::MovementPlugin;
use crate::game::animation::AnimationPlugin;
use crate::game::player::PlayerPlugin;
use bevy::app::App;
use bevy::prelude::{Component, Plugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugins(WindowPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(AnimationPlugin)
            // .add_plugins(GameCameraPlugin)
        ;
    }
}
