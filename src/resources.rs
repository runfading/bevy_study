use crate::configs::{SPRITE_SHEET_H, SPRITE_SHEET_PATH, SPRITE_SHEET_W, TILE_W};
use crate::state::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct ResourcesPlugin;

#[derive(Resource, Default)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct CursorPosition(pub(crate) Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default())
            .insert_resource(CursorPosition(None))
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                (update_cursor_position,).run_if(in_state(GameState::GameInit)),
            );
    }
}

fn load_assets(
    mut handle: ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    handle.image = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W, TILE_W),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    handle.layout = Some(texture_atlas_layout.add(layout));

    game_state.set(GameState::GameInit);
}

fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    // 获取相机和相机坐标
    let (camera, camera_transform) = if let Ok((camera, camera_transform)) = camera_query.single() {
        (camera, camera_transform)
    } else {
        return;
    };

    // 查主窗口
    let window = if let Ok(window) = window_query.single() {
        window
    } else {
        return;
    };

    // 检查光标是否在窗口内并获取其位置
    // 然后，要求bevy转换为世界坐标，并截断以丢弃Z
    cursor_position.0 = window
        .cursor_position()
        .and_then(|cursor| {
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) {
                Some(ray)
            } else {
                None
            }
        })
        .map(|ray| ray.origin.truncate())
}
