//! Asset loading

use bevy::prelude::*;

pub mod assets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(assets::plugin);
}
