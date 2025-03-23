use bevy::asset::AssetServer;
use bevy::prelude::*;
use crate::plugins::camera::systems::camera_system::CameraController;

#[derive(Component)]
pub struct SpeedDisplay;

/// Spawns a UI Text entity to display speed/positions.
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Use the new UI API, or the old UI Node-based system. 
    // This example uses an older approach to Node/Style, but can be adapted to `TextBundle`.
    // If you're on Bevy 0.11+, you can also do `TextBundle::from_section(...)`.
    commands.spawn((
        // The text to display:
        Text::new("Speed: 0.0"),
        // The font, loaded from an asset file
        TextFont {
            font: asset_server.load("fonts/minecraft_font.ttf"),
            font_size: 25.0,
            ..default()
        },
        // The text layout style
        TextLayout::new_with_justify(JustifyText::Left),
        // Style for positioning the UI node
        Node {
            position_type: PositionType::Relative,
            bottom: Val::Px(9.0),
            right: Val::Px(9.0),
            ..default()
        },
        // Our marker so we can query this entity
        SpeedDisplay,
    ));
}

/// System that updates the UI text each frame with 
///  - speed
///  - camera f32 position
///  - camera global f64 position
///  - current chunk coordinate
pub fn update(
    // Query the camera controller so we can see its speed
    query_camera_controller: Query<&CameraController>,
    // We also query for the camera's f32 `Transform` and the double `DoubleTransform`
    camera_query: Query<(&Transform, &Camera)>,

    // The UI text entity
    mut query_text: Query<&mut Text, With<SpeedDisplay>>,
) {
    let camera_controller = query_camera_controller.single();
    let (transform, _camera) = camera_query.single();
    let mut text = query_text.single_mut();



    // Format the string to show speed, positions, and chunk coords
    text.0 = format!(
        "\n  Speed: {:.3}\n  Position(f32): ({:.2},{:.2},{:.2})",
        camera_controller.speed,
        transform.translation.x,
        transform.translation.y,
        transform.translation.z,
    );
}