// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/tensor.fbs".

#pragma once

#include "../collection.hpp"
#include "../components/tensor_data.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: A generic n-dimensional Tensor.
    ///
    /// Since the underlying `rerun::datatypes::TensorData` uses `rerun::Collection` internally,
    /// data can be passed in without a copy from raw pointers or by reference from `std::vector`/`std::array`/c-arrays.
    /// If needed, this "borrow-behavior" can be extended by defining your own `rerun::CollectionAdapter`.
    ///
    /// ## Example
    ///
    /// ### Simple Tensor
    /// ![image](https://static.rerun.io/tensor_simple/baacb07712f7b706e3c80e696f70616c6c20b367/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <algorithm> // std::generate
    /// #include <random>
    /// #include <vector>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_tensor_simple");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     std::default_random_engine gen;
    ///     // On MSVC uint8_t distributions are not supported.
    ///     std::uniform_int_distribution<int> dist(0, 255);
    ///
    ///     std::vector<uint8_t> data(8 * 6 * 3 * 5);
    ///     std::generate(data.begin(), data.end(), [&] { return static_cast<uint8_t>(dist(gen)); });
    ///
    ///     rec.log(
    ///         "tensor",
    ///         rerun::Tensor({8, 6, 3, 5}, data).with_dim_names({"width", "height", "channel", "batch"})
    ///     );
    /// }
    /// ```
    struct Tensor {
        /// The tensor data
        rerun::components::TensorData data;

      public:
        static constexpr const char IndicatorComponentName[] = "rerun.components.TensorIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        // Extensions to generated type defined in 'tensor_ext.cpp'

        /// New Tensor from dimensions and tensor buffer.
        Tensor(Collection<datatypes::TensorDimension> shape, datatypes::TensorBuffer buffer)
            : Tensor(datatypes::TensorData(std::move(shape), std::move(buffer))) {}

        /// New tensor from dimensions and pointer to tensor data.
        ///
        /// Type must be one of the types supported by `rerun::datatypes::TensorData`.
        /// \param shape
        /// Shape of the image. Determines the number of elements expected to be in `data`.
        /// \param data_
        /// Target of the pointer must outlive the archetype.
        template <typename TElement>
        explicit Tensor(Collection<datatypes::TensorDimension> shape, const TElement* data_)
            : Tensor(datatypes::TensorData(std::move(shape), data_)) {}

        /// Update the `names` of the contained `TensorData` dimensions.
        ///
        /// Any existing Dimension names will be overwritten.
        ///
        /// If too many, or too few names are provided, this function will call
        /// Error::handle and then proceed to only update the subset of names that it can.
        Tensor with_dim_names(Collection<std::string> names) &&;

      public:
        Tensor() = default;
        Tensor(Tensor&& other) = default;

        explicit Tensor(rerun::components::TensorData _data) : data(std::move(_data)) {}

        /// Returns the number of primary instances of this archetype.
        size_t num_instances() const {
            return 1;
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::Tensor> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(const archetypes::Tensor& archetype);
    };
} // namespace rerun
