use bevy::prelude::{Component, Resource};
use bevy::reflect::List;

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct Body;

#[derive(Component)]
pub struct Tail;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct BodyResource {
    body: Vec<Body>,
}
