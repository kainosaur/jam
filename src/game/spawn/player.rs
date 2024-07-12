//! Spawn the player.

use std::time::Duration;

use bevy::prelude::*;

use crate::{
    game::{
        asset_loading::assets::ImageAssets,
        movement::{Movement, MovementController, StepSfx, WrapWithinWindow},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: image_assets.ducky.clone(),
            transform: Transform::from_scale(Vec3::splat(0.5)),
            ..Default::default()
        },
        MovementController::default(),
        Movement { speed: 420.0 },
        WrapWithinWindow,
        StepSfx::new(Duration::from_millis(250)),
        StateScoped(Screen::Playing),
    ));
}
