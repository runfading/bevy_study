use crate::assets::{AnimationData, AnimationsResource};
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct PlayerData {
    image_name: String,
    animations: Vec<PlayerAnimations>,
}

#[derive(Deserialize)]
pub(super) struct PlayerAnimations {
    pub(super) form_name: String,
    pub(super) action_frames: HashMap<String, Vec<SpriteFrame>>,
}
/// sprite数据
#[derive(Deserialize)]
pub(super) struct SpriteFrame {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

// 从JSON加载所有动画
pub fn load_mario_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 加载JSON数据
    let player_data: PlayerData = {
        let json_content = std::fs::read_to_string("assets/mario/source/data/player/mario.json")
            .expect("无法读取Mario JSON文件");
        serde_json::from_str::<PlayerData>(&json_content).expect("无法解析Mario JSON")
    };

    // 加载sprite
    let texture_path = format!("mario/resources/graphics/{}.png", player_data.image_name);
    let _texture = asset_server.load::<Image>(&texture_path);

    let mut animations = AnimationsResource::new();

    // 为每个动作创建动画数据
    for player_animation in &player_data.animations {
        let mut max_x = 0;
        let mut max_y = 0;

        let name = player_animation.form_name.as_str();
        // 添加每一帧到布局
        for (action_name, infos) in player_animation.action_frames.iter() {
            let mut sprites = Vec::new();
            for frame in infos.iter() {
                sprites.push(URect::new(
                    frame.x,
                    frame.y,
                    frame.x + frame.width,
                    frame.y + frame.height,
                ));

                max_x = max_x.max(frame.x + frame.width);
                max_y = max_y.max(frame.y + frame.height);
            }
            let animation_name = format!("{}_{}", name, action_name);

            let len = sprites.len();
            let layout = TextureAtlasLayout {
                size: UVec2::new(max_x, max_y),
                textures: sprites,
            };

            // 存储布局
            let layout_handle = texture_atlas_layouts.add(layout);

            // 创建动画数据
            animations.animations.insert(
                animation_name,
                AnimationData {
                    layout: layout_handle,
                    first_index: 0,
                    last_index: len - 1,
                },
            );
        }
    }
    // 将动画集合添加为资源
    commands.insert_resource(animations);
}
