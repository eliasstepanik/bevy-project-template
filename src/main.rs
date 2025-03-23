mod plugins;
mod app;
mod helper;
use bevy::DefaultPlugins;
use bevy::gizmos::{AppGizmoBuilder, GizmoPlugin};
use bevy::log::info;
use bevy::prelude::{default, App, GizmoConfigGroup, PluginGroup, Reflect, Res, Resource};
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy_egui::{EguiPlugin};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_window::{PresentMode, Window, WindowPlugin};
use crate::app::AppPlugin;

const TITLE: &str = "{{project-name}}";
const RESOLUTION: (f32,f32) = (1920f32, 1080f32);
const RESIZABLE: bool =  true;
const DECORATIONS: bool =  true;
const TRANSPARENT: bool =  true;
const PRESENT_MODE: PresentMode = PresentMode::AutoVsync;



fn main() {
    let mut app = App::new();
    register_platform_plugins(&mut app);

    app.add_plugins(AppPlugin);
    app.add_plugins(EguiPlugin);
    app.add_plugins(DefaultInspectorConfigPlugin);
    /*app.add_plugins(GizmoPlugin);*/

    app.run();
}



#[derive(Resource)]
pub struct InspectorVisible(bool);
fn register_platform_plugins(app: &mut App) {
    #[cfg(target_os = "windows")]
    {
        // Register Windows-specific plugins
        info!("Adding Windows-specific plugins");
        app.add_plugins(DefaultPlugins
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.to_string(), // Window title
                    resolution: RESOLUTION.into(), // Initial resolution (width x height)
                    resizable: RESIZABLE,                   // Allow resizing
                    decorations: DECORATIONS,                 // Enable window decorations
                    transparent: TRANSPARENT,                // Opaque background
                    present_mode: PRESENT_MODE, // VSync mode
                    ..default()
                }),
                ..default()
            })
        );
    }

    #[cfg(target_os = "macos")]
    {
        info!("Adding macOS-specific plugins");
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                title: crate::TITLE.to_string(), // Window title
                resolution: crate::RESOLUTION.into(), // Initial resolution (width x height)
                resizable: crate::RESIZABLE,                   // Allow resizing
                decorations: crate::DECORATIONS,                 // Enable window decorations
                transparent: crate::TRANSPARENT,                // Opaque background
                present_mode: crate::PRESENT_MODE, // VSync mode
                ..default() }),
                ..default()
            }));
    }
}
fn should_display_inspector(inspector_visible: Res<InspectorVisible>) -> bool {
    inspector_visible.0
}
