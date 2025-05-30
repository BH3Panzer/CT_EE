use std::f32::consts::PI;

use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::pbr::{CascadeShadowConfigBuilder};
use bevy::text::FontSmoothing;
use bevy::{prelude::*, window::PresentMode};
use ressource::RessourcePlugin;
mod map;
mod camera;
mod audio;
mod ressource;
use crate::camera::*;
use crate::map::*;
use crate::audio::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ChronoTech: Evolution of Empires".into(),
                name: Some("ChronoTech: Evolution of Empires".into()),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::AutoNoVsync,
                resizable: true,
                ..default()
            }),
            ..default()
            
        }))
        .add_plugins(FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        // Here we define size of our overlay
                        font_size: 18.0,
                        // If we want, we can use a custom font
                        font: default(),
                        // We could also disable font smoothing,
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },
                    // We can also change color of the overlay
                    text_color: Color::srgb(0., 1.0, 0.),
                    // We can also set the refresh interval for the FPS counter
                    refresh_interval: core::time::Duration::from_millis(500),
                    enabled: true,
                },
            },
        )
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(RessourcePlugin)
        .add_plugins(AudioPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 2_500.,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(PI/6. * 8.)),
    ));
}