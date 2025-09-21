use crate::ui_component::button::{spawn_button_bundle, ButtonCallbacks, GeneralStruct};
use crate::ui_component::{ButtonPlugins, UiTheme};
use bevy::app::Startup;
use bevy::color::Color;
use bevy::prelude::{
    default, AlignItems, App, BackgroundColor, Camera2d, ClearColor, Commands,
    FlexDirection, JustifyContent, Node, Res, ResMut, Update, Val,
};
use bevy::DefaultPlugins;

mod menu;
pub mod ui_component;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::WHITE))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .add_plugins(ButtonPlugins)
        .add_systems(Update, spawn_menu)
        .run();
}

fn spawn_menu(
    mut commands: Commands,
    theme: Res<UiTheme>,
    asset_server: Res<bevy::asset::AssetServer>,
    mut button_callbacks: ResMut<ButtonCallbacks>,
) {
    let handler = asset_server.load("fonts/LXGWWenKaiMono-Medium.ttf");

    let mut buttons = vec![];

    for label in vec!["开始游戏", "选项", "退出游戏"] {
        buttons.push(spawn_button_bundle(
            &mut commands,
            GeneralStruct::new(label, handler.clone(), theme.button_theme.clone()),
            button_callbacks.as_mut(),
            |_commands| println!("我触发了{}", label.to_string()),
        ));
    }

    // 创建全屏容器，用于居中显示菜单
    let container = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE), // 透明背景
        ))
        .id();

    let menu_panel = commands
        .spawn((
            Node {
                width: Val::Auto,
                height: Val::Auto,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                // padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(theme.bg_color),
        ))
        .add_children(&buttons)
        .id();

    commands.entity(container).add_children(&[menu_panel]);
}
