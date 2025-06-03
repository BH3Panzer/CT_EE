use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_camera);
        app.add_systems(Update, move_camera);
    }
}

#[derive(Component)]
struct MousePos {
    x: f32,
    y: f32
}

fn build_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 30., 15.).looking_at(Vec3 { x: 0., y: 0., z: 0. }, Vec3::Y),
        MousePos {
            x: 0.,
            y: 0.
        }
    ));
}

fn move_camera(query: Query<(&mut Transform, &mut MousePos), With<Camera3d>>, time: Res<Time>, mut mouse_wheel_events: EventReader<MouseWheel>, mut mouse: EventReader<CursorMoved>, windows: Query<&Window>) {
    let mut width: f32 = 0.;
    let mut height: f32 = 0.;
    for window in windows {
        width = window.width();
        height = window.height();
    }
    for mut components in query {
        
        for event in mouse.read() {
            components.1.x = event.position.x;
            components.1.y = event.position.y;
        }


        for event in mouse_wheel_events.read() {
            if event.y > 0. && components.0.translation.y != 2. {
                components.0.translation.y -= 1500. * time.delta_secs();
                components.0.translation.z -= 750. * time.delta_secs();
            } else if event.y < 0. && components.0.translation.y != 80. {
                components.0.translation.y += 1500. * time.delta_secs();
                components.0.translation.z += 750. * time.delta_secs();
            }
        }

        if components.1.x > width - 40. {
            components.0.translation.x += 40. * time.delta_secs();
        } else if components.1.x > width - 80. {
            components.0.translation.x += 25. * time.delta_secs();
        }

        if components.1.x < 40. {
            components.0.translation.x -= 40. * time.delta_secs();
        } else if components.1.x < 80. {
            components.0.translation.x -= 25. * time.delta_secs();
        }

        if components.1.y < 40. {
            components.0.translation.z -= 40. * time.delta_secs();
        } else if components.1.y < 80. {
            components.0.translation.z -= 25. * time.delta_secs();
        }

        if components.1.y > height - 40. {
            components.0.translation.z += 40. * time.delta_secs();
        } else if components.1.y > height - 80. {
            components.0.translation.z += 25. * time.delta_secs();
        }

        if components.0.translation.y < 2. {
            components.0.translation.y = 2.;
        } else if components.0.translation.y > 80. {
            components.0.translation.y = 80.;
        }

    }
}