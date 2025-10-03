use crate::assets::AnimationsResource;
use crate::game::animation::utils::get_player_animation_name;
use crate::game::animation::{AnimationLastIndex, AnimationTimer};
use crate::game::player::{
    Player, PlayerActonState, PlayerFormState, PlayerSizeState, PlayerStateChange,
};
use crate::GameState;
use bevy::app::App;
use bevy::prelude::{
    in_state, IntoScheduleConfigs, MessageReader, Plugin, Query, Res, Sprite, Update, With,
};

pub(super) struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (play_player_animation, player_animation_change_event)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn play_player_animation(
    mut animation_timer_query: Query<
        (&mut AnimationTimer, &mut Sprite, &mut AnimationLastIndex),
        (With<AnimationTimer>, With<Player>),
    >,
) {
    animation_timer_query
        .iter_mut()
        .for_each(|(timer, mut sprite, animation_last_index)| {
            if timer.0.is_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = (atlas.index + 1) % animation_last_index.0;
                }
            }
        })
}

fn player_animation_change_event(
    mut state_change: MessageReader<PlayerStateChange>,
    mut player_query: Query<
        (
            &mut Sprite,
            &mut AnimationLastIndex,
            &PlayerActonState,
            &PlayerSizeState,
            &PlayerFormState,
        ),
        With<Player>,
    >,
    animation_resource: Res<AnimationsResource>,
) {
    let (mut sprite, mut animation_last_index, animation_name) =
        if let Ok(tuple) = player_query.single_mut() {
            (
                tuple.0,
                tuple.1,
                get_player_animation_name(tuple.2, tuple.3, tuple.4),
            )
        } else {
            return;
        };

    for _ in state_change.read() {
        if let Some(animation) = animation_resource.animations.get(&animation_name) {
            if let Some(atlas) = &mut sprite.texture_atlas {
                // 切换并初始化动画
                atlas.layout = animation.layout.clone();
                atlas.index = 0;
                animation_last_index.0 = animation.last_index + 1;
            }
        }
    }
}
