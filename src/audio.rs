use bevy::prelude::*;
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_title_theme);
    }
}

fn start_title_theme(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(
        AudioPlayer::new(
            asset_server.load("audio/CT_EE - Title.ogg")
        )
    );
}