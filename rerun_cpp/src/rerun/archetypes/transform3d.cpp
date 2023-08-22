// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/archetypes/transform3d.fbs"

#include "transform3d.hpp"

#include "../components/transform3d.hpp"

namespace rerun {
    namespace archetypes {
        Result<std::vector<rerun::DataCell>> Transform3D::to_data_cells() const {
            std::vector<rerun::DataCell> cells;
            cells.reserve(1);

            {
                const auto result = rerun::components::Transform3D::to_data_cell(&transform, 1);
                if (result.is_err()) {
                    return result.error;
                }
                cells.emplace_back(std::move(result.value));
            }

            return cells;
        }
    } // namespace archetypes
} // namespace rerun