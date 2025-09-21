use crate::ui_component::button::{
    button_system, handle_button_clicks, ButtonCallbacks, UiButtonPressed,
};
use bevy::color::Color;
use bevy::prelude::*;

pub mod button;

#[derive(Resource, Clone)]
pub struct UiTheme {
    pub bg_color: Color,
    pub text_color: Color,
    pub button_theme: ButtonTheme,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            bg_color: Color::WHITE,
            text_color: Color::BLACK,
            button_theme: Default::default(),
        }
    }
}

impl UiTheme {
    pub fn button_theme(&self) -> &ButtonTheme {
        &self.button_theme
    }
}

#[derive(Component, Clone)]
pub struct ButtonTheme {
    pub border_color: Color,
    pub bg_color: Color,
    pub text_color: Color,

    pub hover_bg_color: Color,
    pub hover_border_color: Color,
    pub hover_text_color: Color,

    pub pressed_bg_color: Color,
    pub pressed_border_color: Color,
    pub pressed_text_color: Color,
}

impl Default for ButtonTheme {
    fn default() -> Self {
        Self {
            bg_color: Color::srgb_u8(247, 248, 250),
            border_color: Color::srgb_u8(247, 248, 250),
            text_color: Color::BLACK,

            hover_border_color: Color::srgb_u8(234, 235, 237),
            hover_bg_color: Color::srgb_u8(234, 235, 237),
            hover_text_color: Color::BLACK,

            pressed_bg_color: Color::srgb_u8(234, 235, 237),
            pressed_border_color: Color::srgb_u8(255, 255, 255),
            pressed_text_color: Color::BLACK,
        }
    }
}

pub struct ButtonPlugins;

impl Plugin for ButtonPlugins {
    fn build(&self, app: &mut App) {
        app.add_event::<UiButtonPressed>()
            .insert_resource(ButtonCallbacks::default())
            .add_systems(Update, (button_system, handle_button_clicks));
    }
}
