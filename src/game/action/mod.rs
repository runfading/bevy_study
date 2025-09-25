use crate::game::action::movement::movement;
use crate::GameState;
use bevy::app::App;
use bevy::prelude::{in_state, IntoScheduleConfigs, Plugin, Update};

mod movement;

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement.run_if(in_state(GameState::InGame)));
    }
}
