

use bevy::{audio::Volume, prelude::*};
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_title_theme);
    }
}

fn start_title_theme(asset_server: Res<AssetServer>, mut commands: Commands, mut volume: ResMut<GlobalVolume>) {
    commands.spawn(
        AudioPlayer::new(
            asset_server.load("audio/CT_EE - Title.ogg")
        ),
    );
    volume.volume = Volume::Linear(0.);
}