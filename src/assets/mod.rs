use crate::GameState;
use bevy::app::{App, PreStartup, Update};
use bevy::asset::{AssetServer, Handle, LoadState};
use bevy::image::{Image, TextureAtlasLayout};
use bevy::log::{error, info};
use bevy::prelude::{
    in_state, Font, IntoScheduleConfigs, NextState, Plugin, Res, ResMut, Resource, Startup,
};
use std::collections::HashMap;

pub mod mario_animations;

#[derive(Resource, Debug, Default)]
pub struct AssetsLoading {
    pub scenes: Vec<Handle<Image>>,
    pub fonts: Vec<Handle<Font>>,
}

/// 字体
#[derive(Resource, Debug, Default)]
pub struct FontAssets {
    pub fonts: Handle<Font>,
}

/// 图片
#[derive(Resource, Debug, Default)]
pub struct ImageAssets {
    pub role: Handle<Image>,
}

// 存储所有Mario动画的资源
#[derive(Resource)]
pub struct AnimationsResource {
    pub animations: HashMap<String, AnimationData>,
}

// 单个动画的数据
pub struct AnimationData {
    pub layout: Handle<TextureAtlasLayout>,
    pub first_index: usize,
    pub last_index: usize,
    pub fps: u8,
}

impl AnimationsResource {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }

    // 根据状态名称获取动画数据
    pub fn get(&self, name: &str) -> Option<&AnimationData> {
        self.animations.get(name)
    }
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontAssets>()
            .init_resource::<ImageAssets>()
            .init_resource::<AssetsLoading>()
            .add_systems(
                PreStartup,
                (load_assets, mario_animations::load_mario_animations),
            )
            .add_systems(
                Update,
                check_assets_loaded.run_if(in_state(GameState::AssetLoading)),
            );
    }
}

fn load_assets(
    mut scene_assets: ResMut<ImageAssets>,
    mut font_assets: ResMut<FontAssets>,
    mut loading: ResMut<AssetsLoading>,
    asset_server: Res<AssetServer>,
) {
    // 模型
    *scene_assets = ImageAssets {
        role: asset_server.load("mario/resources/graphics/mario_bros.png"),
    };

    // 字体
    *font_assets = FontAssets {
        fonts: asset_server.load("fonts/LXGWWenKaiMono-Medium.ttf"),
    };

    // 把 handle 存入 loading
    loading.scenes = vec![scene_assets.role.clone()];
    loading.fonts = vec![font_assets.fonts.clone()];
}

fn check_assets_loaded(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // 场景
    loading
        .scenes
        .retain(|handle| match asset_server.get_load_state(handle) {
            Some(LoadState::Loaded) => {
                info!("Loaded scene asset: {:?}", handle);
                false
            }
            Some(LoadState::Failed(err)) => {
                error!("Failed to load scene asset: {:?}", err);
                false
            }
            _ => true,
        });

    // 字体
    loading
        .fonts
        .retain(|handle| match asset_server.get_load_state(handle) {
            Some(LoadState::Loaded) => false,
            Some(LoadState::Failed(err)) => {
                error!("Failed to load font asset: {:?}", err);
                false
            }
            _ => true,
        });

    // 如果都空了，说明加载完毕
    if loading.scenes.is_empty() && loading.fonts.is_empty() {
        next_state.set(GameState::MainMenu);
    }
}
