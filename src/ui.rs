use bevy::{prelude::*};
use crate::map::Map;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_ui);
        app.add_systems(Update, update_map_name_ui);
    }
}


#[derive(Component)]
struct UIElement;


fn create_ui(mut commands: Commands) {
    // Create Map Name UI
    commands.spawn((
        UIElement,
        Text::new("Map: NoMapNameLoaded"),
        TextFont {
            ..default()
        },
        TextShadow::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.),
            right: Val::Px(10.),
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor {
            0: Color::srgb(1.0, 0.0, 0.0)
        }
        

    ));
}

fn update_map_name_ui(mut query: Query<(&mut Text, &mut TextColor), With<UIElement>>, query2: Query<&Map>) {
    for components2 in query2 {
        let text = Text::new("Map: ".to_owned() + &components2.name.clone());
        let color = &TextColor {
                0: Color::srgb(1.0, 1.0, 1.0)
            };
        for mut components in query.iter_mut() {
            if components.1.0 == Color::srgb(1.0, 0.0, 0.0) {
                *components.0 = text.clone();
                *components.1 = color.clone();
            }
            
        }
    }
}