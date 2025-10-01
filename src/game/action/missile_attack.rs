use crate::game::action::AttackAction;
use crate::GameState;
use bevy::prelude::*;

pub(super) struct MissilePlugin;
impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (missile_attack, missile_action).run_if(in_state(GameState::InGame)),
        );
    }
}

/// 导弹模组
#[derive(Component)]
pub(crate) struct Missile {
    // 导弹出现的位置（相对于实体）
    pub(crate) missile_position: Vec3,
    pub(crate) missile_speed: f32,
}

// 导弹移动模组
#[derive(Component)]
struct MissileAction {
    // 移动向量
    direction: Vec3,
}

/// 包含攻击模组和导弹模组的实体，可以发射导弹进行攻击
fn missile_attack(
    mut commands: Commands,
    mut query: Query<(&Transform, &Missile), (With<AttackAction>, With<Missile>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        query.iter().for_each(|(transform, missile)| {
            // 1. 把导弹的相对位置转为世界坐标
            let local_offset = missile.missile_position;
            let world_offset = transform.rotation * local_offset;
            let spawn_position = transform.translation + world_offset;

            // 2. 获取前方方向（取 Z 轴为前）
            let forward = transform.rotation * Vec3::Z;

            // 3. 计算速度向量
            let velocity = forward.normalize() * missile.missile_speed;

            // 4. 生成导弹实体
            commands.spawn((
                MissileAction {
                    direction: velocity, // 保存速度向量
                },
                Transform::from_translation(spawn_position),
            ));
        });
    }
}

/// 实体导弹和导弹移动模组，进行移动
fn missile_action(
    mut query: Query<(&mut Transform, &MissileAction), With<MissileAction>>,
    time: Res<Time>,
) {
    for (mut transform, action) in &mut query {
        transform.translation += action.direction * time.delta_secs();
    }
}
