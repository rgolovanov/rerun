---
title: "TimeSeriesScalar"
---

Log a double-precision scalar that will be visualized as a time-series plot.

The current simulation time will be used for the time/X-axis, hence scalars
cannot be timeless!

## Components

**Required**: [`Scalar`](../components/scalar.md)

**Recommended**: [`Radius`](../components/radius.md), [`Color`](../components/color.md)

**Optional**: [`Text`](../components/text.md), [`ScalarScattering`](../components/scalar_scattering.md)

## Links
 * 🌊 [C++ API docs for `TimeSeriesScalar`](https://ref.rerun.io/docs/cpp/stable/structrerun_1_1archetypes_1_1TimeSeriesScalar.html)
 * 🐍 [Python API docs for `TimeSeriesScalar`](https://ref.rerun.io/docs/python/stable/common/archetypes#rerun.archetypes.TimeSeriesScalar)
 * 🦀 [Rust API docs for `TimeSeriesScalar`](https://docs.rs/rerun/latest/rerun/archetypes/struct.TimeSeriesScalar.html)

## Examples

### Simple line plot

code-example: scalar_simple

<center>
<picture>
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1200w.png">
  <img src="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/full.png" width="640">
</picture>
</center>

### Multiple time series plots

code-example: scalar_multiple_plots

<center>
<picture>
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/scalar_multiple/15845c2a348f875248fbd694e03eabd922741c4c/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/scalar_multiple/15845c2a348f875248fbd694e03eabd922741c4c/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/scalar_multiple/15845c2a348f875248fbd694e03eabd922741c4c/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/scalar_multiple/15845c2a348f875248fbd694e03eabd922741c4c/1200w.png">
  <img src="https://static.rerun.io/scalar_multiple/15845c2a348f875248fbd694e03eabd922741c4c/full.png" width="640">
</picture>
</center>

