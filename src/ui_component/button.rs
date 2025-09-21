use crate::ui_component::{ButtonTheme, UiTheme};
use bevy::prelude::*;
use std::collections::HashMap;
use std::ops::Add;

#[derive(Event)]
pub struct UiButtonPressed {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct ButtonCallbacks {
    pub map: HashMap<Entity, Box<dyn FnMut(&mut Commands) + Send + Sync>>,
}

#[derive(Bundle)]
pub struct ButtonBundle {
    pub button: Button,
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
    pub border_color: BorderColor,
    pub text: Text,
    pub text_color: TextColor,
    pub text_font: TextFont,
    pub button_theme: ButtonTheme,
}

#[derive(Component)]
pub struct GeneralStruct {
    pub label: String,
    pub font_handler: Handle<Font>,
    pub button_theme: ButtonTheme,
}

impl GeneralStruct {
    pub fn new(
        label: impl Into<String>,
        font_handler: Handle<Font>,
        button_theme: ButtonTheme,
    ) -> Self {
        Self {
            label: label.into(),
            font_handler,
            button_theme,
        }
    }

    pub fn from_ui_theme(
        label: impl Into<String>,
        font_handler: Handle<Font>,
        ui_theme: &UiTheme,
    ) -> Self {
        Self {
            label: label.into(),
            font_handler,
            button_theme: ui_theme.button_theme.clone(),
        }
    }
}

/// 生成具有给定标签和主题的按钮bundle
pub fn create_button_bundle(button: GeneralStruct) -> impl Bundle {
    let theme = button.button_theme;

    ButtonBundle {
        button: Button,
        node: Node {
            width: Val::Px(300.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(theme.bg_color),
        border_radius: BorderRadius::all(Val::Px(5.0)),
        border_color: BorderColor(theme.border_color),
        text: Text::new(button.label),
        text_color: TextColor(theme.text_color),
        text_font: TextFont::from_font(button.font_handler).with_font_size(24.0),
        button_theme: theme,
    }
}

/// 生成具有给定标签和主题的按钮bundle，直接spawn到world中
pub fn spawn_button_bundle<F: FnMut(&mut Commands) + Send + Sync + 'static>(
    commands: &mut Commands,
    general_button: GeneralStruct,
    callbacks: &mut ButtonCallbacks,
    callback: F,
) -> Entity {
    let bundle = create_button_bundle(general_button);
    let button_entity = commands.spawn(bundle).id();

    callbacks.map.insert(button_entity, Box::new(callback));
    button_entity
}

pub fn button_system(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Text,
            &mut TextColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    theme: Res<UiTheme>,
    mut button_pressed_event: EventWriter<UiButtonPressed>,
) {
    let theme = &theme.button_theme;
    for (entity, interaction, mut background_color, mut border_color, mut text, mut text_color) in
        &mut interaction_query
    {
        match interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(theme.pressed_bg_color);
                *text_color = TextColor(theme.pressed_text_color);
                *border_color = BorderColor(theme.pressed_border_color);
                text.0 = text.0.replace(" hover", "").add(" pressed");
                button_pressed_event.write(UiButtonPressed { entity });
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(theme.hover_bg_color);
                *text_color = TextColor(theme.hover_text_color);
                *border_color = BorderColor(theme.hover_border_color);
                text.0 = text.0.replace(" pressed", "").add(" hover");
            }
            Interaction::None => {
                *background_color = BackgroundColor(theme.bg_color);
                *text_color = TextColor(theme.text_color);
                *border_color = BorderColor(theme.border_color);
                text.0 = text.0.replace(" pressed", "").replace(" hover", "");
            }
        }
    }
}

pub fn handle_button_clicks(
    mut events: EventReader<UiButtonPressed>,
    mut callbacks: ResMut<ButtonCallbacks>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let Some(callback) = callbacks.map.get_mut(&event.entity) {
            callback(&mut commands);
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
            .add_event::<UiButtonPressed>()
            .insert_resource(theme.clone())
            .insert_resource(ButtonCallbacks::default())
            .add_systems(Update, button_system)
            .add_systems(Update, handle_button_clicks);

        // 使用新的create_button_bundle函数创建按钮
        let bundle = create_button_bundle(GeneralStruct::from_ui_theme(
            "开始游戏",
            Handle::default(),
            &theme,
        ));

        let button_entity = app.world_mut().spawn(bundle).id();

        // 模拟按钮被按下
        app.world_mut()
            .entity_mut(button_entity)
            .insert(Interaction::Pressed);

        app.update();

        // 验证背景颜色改变
        let bg_color = app.world().get::<BackgroundColor>(button_entity);
        assert_eq!(bg_color.unwrap().0, theme.button_theme.pressed_bg_color);

        // 验证文本颜色改变
        let text_color = app.world().get::<TextColor>(button_entity);
        assert_eq!(text_color.unwrap().0, theme.button_theme.pressed_text_color);
    }
}
