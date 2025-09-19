use crate::ui_component::{ButtonTheme, UiTheme};
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;
use std::ops::Add;

pub(crate) type GeneralButton = (
    Button,
    ButtonAction,
    Node,
    BackgroundColor,
    BorderRadius,
    BorderColor,
    SpawnRelatedBundle<ChildOf, Spawn<(Text, TextColor, TextFont)>>,
);

#[derive(Component)]
pub struct ButtonAction(pub &'static str);

/// 生成具有给定标签和主题的按钮bundle
pub(crate) fn spawn_button_bundle(
    label: &str,
    font_handler: Handle<Font>,
    ui_theme: &UiTheme,
    button_theme: Option<&ButtonTheme>,
    action: &str,
) -> GeneralButton {
    let theme;
    if button_theme.is_none() {
        theme = &ui_theme.button_theme;
    } else {
        theme = button_theme.unwrap();
    }

    (
        Button,
        ButtonAction(action),
        Node {
            width: Val::Px(300.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(theme.bg_color),
        BorderRadius::all(Val::Px(5.0)),
        BorderColor(theme.border_color),
        children![(
            Text::new(label),
            TextColor(theme.text_color),
            TextFont::from_font(font_handler).with_font_size(24.0),
        )],
    )
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut color_query: Query<&mut TextColor>,
    theme: Res<UiTheme>,
) {
    let theme = &theme.button_theme;
    for (interaction, mut background_color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let mut text_color = color_query.get_mut(children[0]).unwrap();
        match interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(theme.pressed_bg_color);
                *text_color = TextColor(theme.pressed_text_color);
                *border_color = BorderColor(theme.pressed_border_color);
                *text = (**text).replace(" hover", "").add(" pressed").into();
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(theme.hover_bg_color);
                *text_color = TextColor(theme.hover_border_color);
                *border_color = BorderColor(theme.hover_text_color);
                *text = (**text).replace(" pressed", "").add(" hover").into();
            }
            Interaction::None => {
                *background_color = BackgroundColor(theme.bg_color);
                *text_color = TextColor(theme.border_color);
                *border_color = BorderColor(theme.text_color);
                *text = (**text)
                    .replace(" pressed", "")
                    .replace(" hover", "")
                    .into();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button() {
        let theme = UiTheme::default();
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(theme.clone())
            .add_systems(Update, button_system);

        let button_entity = app
            .world_mut()
            .spawn(spawn_button_bundle(
                "开始游戏",
                Handle::default(),
                &theme,
                None,
                "开始",
            ))
            .id();

        app.world_mut()
            .entity_mut(button_entity)
            .insert(Interaction::Pressed);

        app.update();

        let bg_color = app.world().get::<BackgroundColor>(button_entity);
        assert_eq!(bg_color.unwrap().0, theme.button_theme.pressed_bg_color);
    }
}
