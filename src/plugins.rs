use bevy::prelude::*;
use bevy_camera_extras::plugins::DefaultCameraPlugin;
use bevy_mod_raycast::RaycastSystem;
use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::pbr::wireframe::WireframePlugin;
use bevy_component_extras::components::*;
use crate::transform_widget::plugins::TransformWidgetPlugin;
use crate::ui::*;
use super::systems::*;
use bevy_ui_extras::systems::*;
pub struct SelecterPlugin;


impl Plugin for SelecterPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
            DefaultRaycastingPlugin::<Selectable>::default(),
            WireframePlugin,
            )
        )
        .add_systems(
            First,update_raycast_with_cursor::<Selectable>.before(RaycastSystem::BuildRays::<Selectable>)
        )
        .add_systems(Update, ( hover_mesh_at_mouse::<Selectable>, manage_selection_behaviour::<Selectable>))
        .add_systems(Update, attach_selector_to_cam::<Selectable>)

        ;
    }
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
            TransformWidgetPlugin,
            SelecterPlugin,
            DefaultCameraPlugin,
            WorldInspectorPlugin::new(),
            )
        )
        .add_systems(Update, (visualize_sidepanel_for::<Selected>, build_menu))
        ;
    }
}