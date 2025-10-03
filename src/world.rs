use crate::animation::AnimationTimer;
use crate::configs::{NUM_WORLD_DECORATIONS, SPRITE_SCALE_FACTOR, WORLD_HEIGHT, WORLD_WIDTH};
use crate::gun::{Gun, GunTimer};
use crate::player::{Player, PlayerState};
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;
use bevy::app::App;
use bevy::image::TextureAtlas;
use bevy::math::{vec3, Vec3};
use bevy::prelude::{Commands, OnEnter, Plugin, Res, Sprite, TimerMode, Transform};
use bevy::time::{Stopwatch, Timer};
use rand::Rng;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_word, spawn_world_decorations),
        );
    }
}

fn init_word(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    commands.spawn((
        Player,
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
        ));
    }
}
