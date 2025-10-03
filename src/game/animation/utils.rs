use crate::game::player::{PlayerActonState, PlayerFormState, PlayerSizeState};

pub(crate) fn get_player_animation_name(
    acton_state: &PlayerActonState,
    size_state: &PlayerSizeState,
    form_state: &PlayerFormState,
) -> String {
    let action_state = match acton_state {
        PlayerActonState::Idle => "idle",
        PlayerActonState::Walk => "walk",
        PlayerActonState::Run => "run",
        PlayerActonState::Jump => "jump",
        PlayerActonState::Stop => "stop",
        PlayerActonState::Die => "die",
        PlayerActonState::Squat => "squat",
    };

    let size_state = match size_state {
        PlayerSizeState::Small => "small",
        PlayerSizeState::Big => "big",
    };

    let form_state = match form_state {
        PlayerFormState::Normal => "normal",
        PlayerFormState::DiffNormal => "diff_normal",
        PlayerFormState::Fire => "fire",
        PlayerFormState::DiffFire => "diff_fire",
        PlayerFormState::Star => "star",
        PlayerFormState::DiffStar => "diff_star",
        PlayerFormState::UnderGroud => "under_groud",
        PlayerFormState::DiffUnderGroud => "diff_under_groud",
        PlayerFormState::Castle => "castle",
        PlayerFormState::DiffCastle => "diff_castle",
        PlayerFormState::UnderWater => "underwater",
        PlayerFormState::DiffUnderWater => "diff_under_water",
    };

    format!("{}_{}_{}", size_state, form_state, action_state)
}
