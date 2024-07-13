use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/ducky.png")]
    pub ducky: Handle<Image>,
    #[asset[path = "images/splash.png"]]
    // Unable to make splash linear due to bug
    #[asset(image(sampler = linear))]
    pub splash: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    //sfx
    #[asset(path = "audio/sfx/button_hover.ogg")]
    pub button_hover: Handle<AudioSource>,
    #[asset(path = "audio/sfx/button_press.ogg")]
    pub button_press: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step1.ogg")]
    pub step1: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step2.ogg")]
    pub step2: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step3.ogg")]
    pub step3: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step4.ogg")]
    pub step4: Handle<AudioSource>,
    //soundtracks
    #[asset(path = "audio/soundtracks/Fluffing A Duck.ogg")]
    pub fluffing_a_duck: Handle<AudioSource>,
    #[asset(path = "audio/soundtracks/Monkeys Spinning Monkeys.ogg")]
    pub monkeys_spinning_monkeys: Handle<AudioSource>,
}
