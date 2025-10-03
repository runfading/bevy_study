use crate::animation::AnimationTimer;
use crate::configs::{NUM_WORLD_DECORATIONS, SPRITE_SCALE_FACTOR, WORLD_HEIGHT, WORLD_WIDTH};
use crate::gun::{Gun, GunTimer};
use crate::player::{PlayHealth, Player, PlayerState};
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;
use crate::PLAYER_HEALTH;
use bevy::app::App;
use bevy::image::TextureAtlas;
use bevy::math::{vec3, Vec3};
use bevy::prelude::{
    Commands, Component, Entity, NextState, OnEnter, OnExit, Plugin, Query, Res, ResMut, Sprite,
    TimerMode, Transform, With,
};
use bevy::time::{Stopwatch, Timer};
use rand::Rng;

pub struct WorldPlugin;

#[derive(Component)]
pub struct GameEntity;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_word, spawn_world_decorations),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all);
    }
}

fn despawn_all(mut commands: Commands, game_entity: Query<Entity, With<GameEntity>>) {
    for e in game_entity.iter() {
        commands.entity(e).despawn();
    }
}

fn init_word(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        Player,
        PlayHealth(PLAYER_HEALTH),
        GameEntity,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        Sprite::from_atlas_image(
            handle.image.clone().unwrap(),
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 0,
            },
        ),
        Transform::from_translation(vec3(0., 0., 10.)).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        PlayerState::Idle,
    ));

    commands.spawn((
        Gun,
        GameEntity,
        GunTimer(Stopwatch::new()),
        Sprite::from_atlas_image(
            handle.image.clone().unwrap(),
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 17,
            },
        ),
        Transform::from_translation(vec3(0., 0., 10.)).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
    ));

    game_state.set(GameState::InGame);
}

fn spawn_world_decorations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::rng();

    for _ in 0..NUM_WORLD_DECORATIONS {
        commands.spawn((
            Transform::from_translation(vec3(
                rng.random_range(-WORLD_WIDTH..WORLD_WIDTH),
                rng.random_range(-WORLD_HEIGHT..WORLD_HEIGHT),
                0.,
            ))
            .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Sprite::from_atlas_image(
                handle.image.clone().unwrap(),
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: rng.random_range(24..=25),
                },
            ),
            GameEntity,
        ));
    }
}
