use bevy::a11y::AccessibilitySystem::Update;
use bevy::app::{App, Plugin, PreUpdate, Startup};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, _app: &mut App) {
        _app.add_systems(Startup, (crate::plugins::camera::systems::camera_system::setup));
        _app.add_systems(PreUpdate, (crate::plugins::camera::systems::camera_system::camera_controller_system));
    }


}
