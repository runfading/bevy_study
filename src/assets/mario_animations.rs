use crate::assets::{AnimationData, AnimationsResource};
use bevy::prelude::*;
use std::cmp::max;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
struct MarioData {
    image_name: String,
    image_frames: HashMap<String, Vec<SpriteFrame>>,
}

/// sprite数据
#[derive(Deserialize)]
struct SpriteFrame {
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
    let mario_data: MarioData = {
        let json_content = std::fs::read_to_string("assets/mario/source/data/player/mario.json")
            .expect("无法读取Mario JSON文件");
        serde_json::from_str::<MarioData>(&json_content).expect("无法解析Mario JSON")
    };

    // 加载sprite
    let texture_path = format!("mario/resources/graphics/{}.png", mario_data.image_name);
    let _texture = asset_server.load::<Image>(&texture_path);

    let mut animations = AnimationsResource::new();

    // 为每个动作创建动画数据
    for (action_name, frames) in &mario_data.image_frames {
        let mut sprites = Vec::new();
        let mut max_x = 0;
        let mut max_y = 0;

        // 添加每一帧到布局
        for frame in frames {
            sprites.push(URect::new(
                frame.x,
                frame.y,
                frame.x + frame.width,
                frame.y + frame.height,
            ));

            max_x = max_x.max(frame.x + frame.width);
            max_y = max_y.max(frame.y + frame.height);
        }

        let layout = TextureAtlasLayout {
            size: UVec2::new(max_x, max_y),
            textures: sprites,
        };

        // 存储布局
        let layout_handle = texture_atlas_layouts.add(layout);

        // 创建动画数据
        animations.animations.insert(
            action_name.clone(),
            AnimationData {
                layout: layout_handle,
                first_index: 0,
                last_index: frames.len() - 1,
                fps: 10, // 默认帧率
            },
        );
    }

    // 将动画集合添加为资源
    commands.insert_resource(animations);
}
