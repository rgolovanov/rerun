//! Rerun Space View utilities
//!
//! Types & utilities for defining Space View classes and communicating with the Viewport.

pub mod blueprint;
pub mod controls;
mod data_query;
mod data_query_blueprint;
mod screenshot;
mod unreachable_transform_reason;

pub use data_query::{DataQuery, EntityOverrideContext, PropertyResolver};
pub use data_query_blueprint::DataQueryBlueprint;
pub use screenshot::ScreenshotMode;
pub use unreachable_transform_reason::UnreachableTransformReason;

// -----------

use re_entity_db::external::re_data_store;

/// Utility for implementing [`re_viewer_context::VisualizerAdditionalApplicabilityFilter`] using on the properties of a concrete component.
#[inline]
pub fn diff_component_filter<T: re_types_core::Component>(
    event: &re_data_store::StoreEvent,
    filter: impl Fn(&T) -> bool,
) -> bool {
    let filter = &filter;
    event.diff.cells.iter().any(|(component_name, cell)| {
        component_name == &T::name()
            && T::from_arrow(cell.as_arrow_ref())
                .map(|components| components.iter().any(filter))
                .unwrap_or(false)
    })
}
