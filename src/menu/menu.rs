use crate::ui_component::button::DEFAULT_BUTTON_THEME;
use crate::ui_component::{
    button::{spawn_button_bundle, ButtonCallbacks, GeneralStruct},
    UiTheme,
};
use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_theme: Option<Res<UiTheme>>,
    mut button_callbacks: ResMut<ButtonCallbacks>,
) {
    let ui_theme = ui_theme
        .as_ref()
        .map(|theme| theme.as_ref())
        .unwrap_or(&DEFAULT_BUTTON_THEME);
    let font_handle = asset_server.load("fonts/LXGWWenKaiMono-Medium.ttf");

    // 创建主菜单容器
    let menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(80.),  // 占窗口宽度80%
                height: Val::Percent(80.), // 占窗口高度80%
                // 居中对齐子元素
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                // ui自动分配，实现居中
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            MainMenu,
        ))
        .with_children(|parent| {
            // 创建按钮容器
            parent
                .spawn(Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|button_parent| {
                    // 开始游戏按钮
                    let start_button =
                        GeneralStruct::from_ui_theme("开始游戏", font_handle.clone(), ui_theme);
                    spawn_button_bundle(
                        button_parent,
                        start_button,
                        &mut button_callbacks,
                        |_commands| {
                            info!("开始游戏按钮被点击");
                            // 这里添加开始游戏的逻辑
                        },
                    );

                    // 选项按钮
                    let options_button =
                        GeneralStruct::from_ui_theme("选项", font_handle.clone(), ui_theme);
                    spawn_button_bundle(
                        button_parent,
                        options_button,
                        &mut button_callbacks,
                        |_commands| {
                            info!("选项按钮被点击");
                            // 这里添加打开选项菜单的逻辑
                        },
                    );

                    // 退出按钮
                    let exit_button =
                        GeneralStruct::from_ui_theme("退出", font_handle.clone(), ui_theme);
                    spawn_button_bundle(
                        button_parent,
                        exit_button,
                        &mut button_callbacks,
                        |_commands| {
                            info!("退出按钮被点击");
                            // 这里添加退出游戏的逻辑
                            std::process::exit(0);
                        },
                    );
                });
        })
        .id();

    info!("主菜单已生成，实体ID: {:?}", menu_entity);
}

// 菜单插件
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
    }
}
