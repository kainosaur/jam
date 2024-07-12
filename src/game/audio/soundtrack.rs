use bevy::{audio::PlaybackMode, prelude::*};

use crate::game::asset_loading::assets::AudioAssets;

pub(super) fn play_soundtrack(
    trigger: Trigger<Soundtrack>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    query: Query<Entity, With<SoundtrackMarker>>,
) {
    let event = trigger.event();
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    let audio_picked = match event {
        Soundtrack::Disable => {
            return;
        }
        Soundtrack::Credits => &audio_assets.monkeys_spinning_monkeys,
        Soundtrack::Gameplay => &audio_assets.fluffing_a_duck,
    };
    let source = audio_picked.clone();
    let settings = PlaybackSettings {
        mode: PlaybackMode::Loop,
        ..default()
    };
    commands.spawn((AudioSourceBundle { source, settings }, SoundtrackMarker));
}

/// We mark our soundtrack entity so we can find it later.
#[derive(Component)]
pub(super) struct SoundtrackMarker;

/// Play or disable the soundtrack.
/// Playing a new soundtrack will overwrite the previous one.
/// Soundtracks will loop.
#[derive(Event)]
pub enum Soundtrack {
    Credits,
    Gameplay,
    Disable,
}
