use bevy::color::Color;
use bevy::prelude::*;

mod button;

#[derive(Resource, Clone)]
pub(crate) struct UiTheme {
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

#[derive(Clone)]
pub(crate) struct ButtonTheme {
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
