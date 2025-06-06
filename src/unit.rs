use bevy::{ecs::system::command, prelude::*};

use crate::{team::Team, unit_type::Citizen};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_citizens_test);
        app.add_systems(Update, update_units_gravity);
    }
}

#[derive(Component)]
struct Unit {
    life: u16,
    weight: f32,
    team: Team
}

fn spawn_citizens_test(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let model: Handle<Mesh> = meshes.add(Cuboid::new(0.15, 0.35, 0.15));
    let material: Handle<StandardMaterial> = materials.add(StandardMaterial::from_color(Color::srgb(0.5, 0.5, 0.5)));
    for i in 0..4 {
        commands.spawn((
            Unit {
                life: 100,
                weight: 0.5,
                team: Team::Blue
            },
            Citizen,
            Transform::from_xyz(i as f32 * 4.0, 5.0, 5.0),
            Mesh3d(model.clone()),
            MeshMaterial3d(material.clone())
        ));
    }
}

fn update_units_gravity(mut query: Query<(&mut Transform, &mut Unit), With<Unit>>, time: Res<Time>) {
    for mut unit in query.iter_mut() {
        if unit.0.translation.y > 0.35/2.0 {
            unit.0.translation.y -= unit.1.weight * 10. * time.delta_secs();
            if unit.0.translation.y < 0.35/2.0 {
                unit.0.translation.y = 0.35/2.0;
            }
        }
    }
}