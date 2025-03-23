use bevy::prelude::*;
use bevy_egui::EguiSet;
use bevy_render::extract_resource::ExtractResourcePlugin;
use crate::helper::debug_gizmos::debug_gizmos;
use crate::helper::egui_dock::{reset_camera_viewport, set_camera_viewport, set_gizmo_mode, show_ui_system, UiState};

pub struct AppPlugin;


#[derive(Resource, Debug)]
pub struct InspectorVisible(pub bool);

impl Default for InspectorVisible {
    fn default() -> Self {
        InspectorVisible(false)
    }
}
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::new());
        app.insert_resource(InspectorVisible(true));



        app.add_plugins(crate::plugins::camera::camera_plugin::CameraPlugin);
        app.add_plugins(crate::plugins::ui::ui_plugin::UiPlugin);

        app.add_plugins(crate::plugins::environment::environment_plugin::EnvironmentPlugin);










        app.add_systems(Update, (debug_gizmos, toggle_ui_system));
        app.add_systems(
            PostUpdate,
            show_ui_system
                .before(EguiSet::ProcessOutput)
                .before(bevy_egui::systems::end_pass_system)
                .before(TransformSystem::TransformPropagate)
                .run_if(should_display_inspector),
        );
        app.add_systems(PostUpdate, (set_camera_viewport.after(show_ui_system).run_if(should_display_inspector), reset_camera_viewport.run_if(should_not_display_inspector).after(set_camera_viewport)));
        app.add_systems(Update, set_gizmo_mode);
        app.register_type::<Option<Handle<Image>>>();
        app.register_type::<AlphaMode>();
    }


}



fn toggle_ui_system(

    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut inspector_visible: ResMut<InspectorVisible>,
){
    // =======================
    // 6) Hide Inspector
    // =======================
    if keyboard_input.just_pressed(KeyCode::F1) {
        inspector_visible.0 = !inspector_visible.0
    }

}


fn should_display_inspector(inspector_visible: Res<InspectorVisible>) -> bool {
    inspector_visible.0
}

fn should_not_display_inspector(inspector_visible: Res<InspectorVisible>) -> bool {
    !inspector_visible.0
}
