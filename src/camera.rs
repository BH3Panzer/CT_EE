use bevy::{input::mouse::{MouseWheel}, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_camera);
        app.add_systems(Update, move_camera);
    }
}

fn build_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 30., 15.).looking_at(Vec3 { x: 0., y: 0., z: 0. }, Vec3::Y)
    ));
}

fn move_camera(query: Query<&mut Transform, With<Camera3d>>, input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut mouse_wheel_events: EventReader<MouseWheel>) {
    for mut trans in query {
        if input.pressed(KeyCode::ArrowRight) {
            trans.translation.x += 50. * time.delta_secs();
        } 
        if input.pressed(KeyCode::ArrowLeft) {
            trans.translation.x -= 50. * time.delta_secs();
        }
        if input.pressed(KeyCode::ArrowDown) {
            trans.translation.z += 50. * time.delta_secs();
        } 
        if input.pressed(KeyCode::ArrowUp) {
            trans.translation.z -= 50. * time.delta_secs();
        }

        for event in mouse_wheel_events.read() {
            if event.y > 0. && trans.translation.y != 5. {
                trans.translation.y -= 1500. * time.delta_secs();
                trans.translation.z -= 750. * time.delta_secs();
            } else if event.y < 0. && trans.translation.y != 80. {
                trans.translation.y += 1500. * time.delta_secs();
                trans.translation.z += 750. * time.delta_secs();
            }
        }

        if trans.translation.y < 5. {
            trans.translation.y = 5.;
        } else if trans.translation.y > 80. {
            trans.translation.y = 80.;
        }

    }
}