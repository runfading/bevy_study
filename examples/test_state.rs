// 在main函数中添加主题资源
use bevy::prelude::*;
use bevy::utils::default;
use std::ops::Add;

// 添加主题资源
#[derive(Resource)]
struct UITheme {
    background_color: Color,
    button_color: Color,
    button_hover_color: Color,
    text_color: Color,
    accent_color: Color,
}

impl Default for UITheme {
    fn default() -> Self {
        Self {
            background_color: Color::WHITE,
            button_color: Color::srgb_u8(235, 236, 240),
            button_hover_color: Color::srgb_u8(234, 235, 237),
            text_color: Color::BLACK,
            accent_color: Color::srgb_u8(255, 250, 237),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "我的游戏".into(),
                resolution: (1024.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::WHITE))
        .init_resource::<UITheme>() // 添加主题资源
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
        .add_systems(Update, button_system)
        .run();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // // 设置全局背景色
    // commands.spawn((
    //     Node {
    //         width: Val::Percent(100.0),
    //         height: Val::Percent(100.0),
    //         position_type: PositionType::Absolute,
    //         ..default()
    //     },
    //     ZIndex(-1),
    //     BackgroundColor(Color::WHITE), // 深蓝色背景
    // ));
}

struct MainMenu;

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, theme: Res<UITheme>) {
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
            BackgroundColor(theme.background_color),
        ))
        .id();
    spawn_button(&mut commands, menu_panel, "开始游戏", &asset_server, &theme);
    spawn_button(&mut commands, menu_panel, "设置", &asset_server, &theme);
    spawn_button(&mut commands, menu_panel, "退出游戏", &asset_server, &theme);

    // 将菜单面板添加到容器中
    commands.entity(container).add_children(&[menu_panel]);
}

const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut color_query: Query<&mut TextColor>,
    theme: Res<UITheme>,
) {
    for (interaction, mut background_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let mut text_color = color_query.get_mut(children[0]).unwrap();
        match interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(PRESSED_BUTTON);
                *text_color = TextColor(Color::WHITE);
                *text = (**text).replace(" hover", "").add(" pressed").into();
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(HOVERED_BUTTON);
                *text_color = TextColor(Color::WHITE);
                *text = (**text).replace(" pressed", "").add(" hover").into();
            }
            Interaction::None => {
                *background_color = BackgroundColor(theme.button_color);
                *text_color = TextColor(theme.text_color);
                *text = (**text)
                    .replace(" pressed", "")
                    .replace(" hover", "")
                    .into();
            }
        }
    }
}

// 修改spawn_button函数使用主题
fn spawn_button(
    commands: &mut Commands,
    parent: Entity,
    label: &str,
    asset_server: &Res<AssetServer>,
    theme: &Res<UITheme>,
) {
    let handler = asset_server.load("fonts/LXGWWenKaiMono-Medium.ttf");
    let button = commands
        .spawn((
            Button,
            Node {
                width: Val::Px(300.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.button_color),
            BorderRadius::all(Val::Px(5.0)),
            BorderColor(theme.accent_color),
            children![(
                Text::new(label),
                TextColor(theme.text_color),
                TextFont::from_font(handler).with_font_size(24.0),
            )],
        ))
        .id();

    commands.entity(parent).add_children(&[button]);
}
