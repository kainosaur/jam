use bevy::{audio::PlaybackMode, prelude::*};
use rand::prelude::SliceRandom;

use crate::game::asset_loading::assets::AudioAssets;

pub(super) fn play_sfx(
    trigger: Trigger<Sfx>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
) {
    let event = trigger.event();
    let audio_picked = match event {
        Sfx::ButtonHover => &audio_assets.button_hover,
        Sfx::ButtonPress => &audio_assets.button_press,
        Sfx::Step => random_step(
            &audio_assets.step1,
            &audio_assets.step2,
            &audio_assets.step3,
            &audio_assets.step4,
        ),
    };
    let source = audio_picked.clone();
    let settings = PlaybackSettings {
        mode: PlaybackMode::Despawn,
        ..default()
    };
    commands.spawn(AudioSourceBundle { source, settings });
}

/// Play a single sound effect.
#[derive(Event)]
pub enum Sfx {
    ButtonHover,
    ButtonPress,
    Step,
}

fn random_step<'a>(
    step1: &'a Handle<AudioSource>,
    step2: &'a Handle<AudioSource>,
    step3: &'a Handle<AudioSource>,
    step4: &'a Handle<AudioSource>,
) -> &'a Handle<AudioSource> {
    [step1, step2, step3, step4]
        .choose(&mut rand::thread_rng())
        .unwrap()
}
