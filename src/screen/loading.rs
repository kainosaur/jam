use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::asset_loading::assets::{AudioAssets, ImageAssets};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::AssetLoading)
            .continue_to_state(Screen::Splash)
            .load_collection::<AudioAssets>()
            .load_collection::<ImageAssets>(),
    );
}

// TODO: Add a loading bar/ some sort of indication that the game is currently loading
