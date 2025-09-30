use crate::GameState;
use bevy::asset::LoadState;
use bevy::ecs::error::info;
use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
}

#[derive(Resource, Debug, Default)]
pub struct FontAssets {
    pub fonts: Handle<Font>,
}

#[derive(Resource, Debug, Default)]
pub struct AssetsLoading {
    pub scenes: Vec<Handle<Scene>>,
    pub fonts: Vec<Handle<Font>>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .init_resource::<FontAssets>()
            .init_resource::<AssetsLoading>()
            .add_systems(PreStartup, load_assets)
            .add_systems(
                Update,
                check_assets_loaded.run_if(in_state(GameState::AssetLoading)),
            );
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    mut font_assets: ResMut<FontAssets>,
    mut loading: ResMut<AssetsLoading>,
    asset_server: Res<AssetServer>,
) {
    // 模型
    *scene_assets = SceneAssets {
        asteroid: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Asteroid.glb")),
        spaceship: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Spaceship.glb")),
        missiles: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Missiles.glb")),
    };

    // 字体
    *font_assets = FontAssets {
        fonts: asset_server.load("fonts/LXGWWenKaiMono-Medium.ttf"),
    };

    // 把 handle 存入 loading
    loading.scenes = vec![
        scene_assets.asteroid.clone(),
        scene_assets.spaceship.clone(),
        scene_assets.missiles.clone(),
    ];
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
