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
                        |commands| {
                            info!("开始游戏按钮被点击");
                            // 这里添加开始游戏的逻辑
                            commands.queue(|world: &mut World| {
                                let mut next = world.resource_mut::<NextState<GameState>>();
                                next.set(GameState::InGame);
                                // let mut query = world.query_filtered::<Entity, With<MainMenu>>();
                                //
                                // if let Ok(main_menu_entity) = query.single_mut(world) {
                                //     world.entity_mut(main_menu_entity).despawn();
                                // }
                            });
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

fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    println!("Despawning main menu...");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn handle_esc_key(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::InGame => {
                // 从游戏状态按 ESC 返回主菜单
                next_state.set(GameState::MainMenu);
                println!("ESC pressed: Returning to Main Menu");
            }
            GameState::MainMenu => {
                // 在主菜单按 ESC 可以退出游戏或其他操作
                println!("ESC pressed in Main Menu");
            }
            _ => {}
        }
    }
}

// 菜单插件
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(Update, handle_esc_key.run_if(in_state(GameState::InGame))); // 添加退出时的清理
    }
}
