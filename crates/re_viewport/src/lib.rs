//! Rerun Viewport Panel
//!
//! This crate provides the central panel that contains all space views.

pub const VIEWPORT_PATH: &str = "viewport";

mod auto_layout;
mod container;
mod space_info;
mod space_view;
mod space_view_entity_picker;
mod space_view_heuristics;
mod space_view_highlights;
mod system_execution;
mod viewport;
mod viewport_blueprint;
mod viewport_blueprint_ui;

/// Auto-generated blueprint-related types.
///
/// They all implement the [`re_types_core::Component`] trait.
///
/// Unstable. Used for the ongoing blueprint experimentations.
pub mod blueprint;

pub use space_info::SpaceInfoCollection;
pub use space_view::{SpaceViewBlueprint, SpaceViewName};
pub use viewport::{Viewport, ViewportState};
pub use viewport_blueprint::ViewportBlueprint;

pub mod external {
    pub use re_space_view;
}

use re_entity_db::EntityDb;
use re_log_types::EntityPath;
use re_types::datatypes;

use re_viewer_context::{
    ApplicableEntities, DynSpaceViewClass, PerVisualizer, VisualizableEntities,
};

/// Utility for querying a pinhole archetype instance.
///
/// TODO(andreas): It should be possible to convert `re_query::ArchetypeView` to its corresponding Archetype for situations like this.
/// TODO(andreas): This is duplicated into `re_space_view_spatial`
fn query_pinhole(
    store: &re_data_store::DataStore,
    query: &re_data_store::LatestAtQuery,
    entity_path: &re_log_types::EntityPath,
) -> Option<re_types::archetypes::Pinhole> {
    store
        .query_latest_component(entity_path, query)
        .map(|image_from_camera| re_types::archetypes::Pinhole {
            image_from_camera: image_from_camera.value,
            resolution: store
                .query_latest_component(entity_path, query)
                .map(|c| c.value),
            camera_xyz: store
                .query_latest_component(entity_path, query)
                .map(|c| c.value),
        })
}

/// Determines the set of visible entities for a given space view.
// TODO(andreas): This should be part of the SpaceView's (non-blueprint) state.
// Updated whenever `applicable_entities_per_visualizer` or the space view blueprint changes.
pub fn determine_visualizable_entities(
    applicable_entities_per_visualizer: &PerVisualizer<ApplicableEntities>,
    entity_db: &EntityDb,
    visualizers: &re_viewer_context::VisualizerCollection,
    class: &dyn DynSpaceViewClass,
    space_origin: &EntityPath,
) -> PerVisualizer<VisualizableEntities> {
    re_tracing::profile_function!();

    let filter_ctx = class.visualizable_filter_context(space_origin, entity_db);

    PerVisualizer::<VisualizableEntities>(
        visualizers
            .iter_with_identifiers()
            .map(|(visualizer_identifier, visualizer_system)| {
                let entities = if let Some(applicable_entities) =
                    applicable_entities_per_visualizer.get(&visualizer_identifier)
                {
                    visualizer_system.filter_visualizable_entities(
                        applicable_entities.clone(),
                        filter_ctx.as_ref(),
                    )
                } else {
                    VisualizableEntities::default()
                };

                (visualizer_identifier, entities)
            })
            .collect(),
    )
}

/// Determines the icon to use for a given container kind.
pub fn icon_for_container_kind(kind: &egui_tiles::ContainerKind) -> &'static re_ui::Icon {
    match kind {
        egui_tiles::ContainerKind::Tabs => &re_ui::icons::CONTAINER_TABS,
        egui_tiles::ContainerKind::Horizontal => &re_ui::icons::CONTAINER_HORIZONTAL,
        egui_tiles::ContainerKind::Vertical => &re_ui::icons::CONTAINER_VERTICAL,
        egui_tiles::ContainerKind::Grid => &re_ui::icons::CONTAINER_GRID,
    }
}
