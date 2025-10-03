mod hide_or_death;
mod mario;

use crate::game::player::hide_or_death::despawn_ball;
use crate::game::player::mario::spawn_or_reset_ball;
use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub(super) struct Player;

#[derive(Component, Default, PartialEq, Eq)]
pub(super) enum PlayerActonState {
    #[default]
    Idle,
    Walk,
    Run,
    Jump,
    Stop,
    Die,
    Squat,
}

#[derive(Component, Default, PartialEq, Eq)]
pub(super) enum PlayerSizeState {
    #[default]
    Small,
    Big,
}

#[derive(Component, Default, PartialEq, Eq)]
pub(super) enum PlayerFormState {
    #[default]
    Normal,
    DiffNormal,
    Fire,
    DiffFire,
    Star,
    DiffStar,
    UnderGroud,
    DiffUnderGroud,
    Castle,
    DiffCastle,
    UnderWater,
    DiffUnderWater,
}

// size action form state 发生改变
#[derive(Message)]
pub(super) struct PlayerStateChange;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerStateChange>()
            .add_systems(OnEnter(GameState::InGame), spawn_or_reset_ball)
            .add_systems(OnExit(GameState::InGame), despawn_ball);
    }
}
