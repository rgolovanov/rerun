use std::sync::atomic::AtomicBool;

use re_data_store::{DataStoreConfig, DataStoreRowStats, DataStoreStats};
use re_format::{format_bytes, format_number};
use re_memory::{util::sec_since_start, MemoryHistory, MemoryLimit, MemoryUse};
use re_query_cache::{CachedComponentStats, CachedEntityStats, CachesStats};
use re_renderer::WgpuResourcePoolStatistics;

use crate::{env_vars::RERUN_TRACK_ALLOCATIONS, store_hub::StoreHubStats};

// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct MemoryPanel {
    history: MemoryHistory,
    memory_purge_times: Vec<f64>,

    /// If `true`, enables the much-more-costly-to-compute per-component stats for the primary
    /// cache.
    prim_cache_detailed_stats: AtomicBool,

    /// If `true`, will show stats about empty primary caches too, which likely indicates a bug (dangling bucket).
    prim_cache_show_empty: AtomicBool,
}

impl MemoryPanel {
    /// Call once per frame
    pub fn update(
        &mut self,
        gpu_resource_stats: &WgpuResourcePoolStatistics,
        store_stats: &StoreHubStats,
        caches_stats: &CachesStats,
    ) {
        re_tracing::profile_function!();
        self.history.capture(
            Some(
                (gpu_resource_stats.total_buffer_size_in_bytes
                    + gpu_resource_stats.total_texture_size_in_bytes) as _,
            ),
            Some(store_stats.recording_stats.total.num_bytes as _),
            Some(caches_stats.total_size_bytes() as _),
            Some(store_stats.blueprint_stats.total.num_bytes as _),
        );
    }

    /// Note that we purged memory at this time, to show in stats.
    #[inline]
    pub fn note_memory_purge(&mut self) {
        self.memory_purge_times.push(sec_since_start());
    }

    #[inline]
    pub fn primary_cache_detailed_stats_enabled(&self) -> bool {
        self.prim_cache_detailed_stats
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn ui(
        &self,
        ui: &mut egui::Ui,
        re_ui: &re_ui::ReUi,
        limit: &MemoryLimit,
        gpu_resource_stats: &WgpuResourcePoolStatistics,
        store_stats: &StoreHubStats,
        caches_stats: &CachesStats,
    ) {
        re_tracing::profile_function!();

        // We show realtime stats, so keep showing the latest!
        ui.ctx().request_repaint();

        egui::SidePanel::left("not_the_plot")
            .resizable(false)
            .min_width(250.0)
            .default_width(300.0)
            .show_inside(ui, |ui| {
                self.left_side(
                    ui,
                    re_ui,
                    limit,
                    gpu_resource_stats,
                    store_stats,
                    caches_stats,
                );
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("🗠 Rerun Viewer memory use over time");
            self.plot(ui, limit);
        });
    }

    fn left_side(
        &self,
        ui: &mut egui::Ui,
        re_ui: &re_ui::ReUi,
        limit: &MemoryLimit,
        gpu_resource_stats: &WgpuResourcePoolStatistics,
        store_stats: &StoreHubStats,
        caches_stats: &CachesStats,
    ) {
        ui.strong("Rerun Viewer resource usage");

        ui.separator();
        ui.collapsing("CPU Resources", |ui| {
            Self::cpu_stats(ui, re_ui, limit);
        });

        ui.separator();
        ui.collapsing("GPU Resources", |ui| {
            Self::gpu_stats(ui, gpu_resource_stats);
        });

        ui.separator();
        ui.collapsing("Datastore Resources", |ui| {
            Self::store_stats(
                ui,
                &store_stats.recording_config,
                &store_stats.recording_stats,
            );
        });

        ui.separator();
        ui.collapsing("Primary Cache Resources", |ui| {
            self.caches_stats(ui, re_ui, caches_stats);
        });

        ui.separator();
        ui.collapsing("Blueprint Resources", |ui| {
            Self::store_stats(
                ui,
                &store_stats.blueprint_config,
                &store_stats.blueprint_stats,
            );
        });
    }

    fn cpu_stats(ui: &mut egui::Ui, re_ui: &re_ui::ReUi, limit: &MemoryLimit) {
        if let Some(max_bytes) = limit.max_bytes {
            ui.label(format!("Memory limit: {}", format_bytes(max_bytes as _)));
        } else {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("You can set an upper limit of RAM use with the command-line option ");
                ui.code("--memory-limit");
            });
            ui.separator();
        }

        let mem_use = MemoryUse::capture();

        if mem_use.resident.is_some() || mem_use.counted.is_some() {
            if let Some(resident) = mem_use.resident {
                ui.label(format!("resident: {}", format_bytes(resident as _)))
                    .on_hover_text("Resident Set Size (or Working Set on Windows). Memory in RAM and not in swap.");
            }

            if let Some(counted) = mem_use.counted {
                ui.label(format!("counted: {}", format_bytes(counted as _)))
                    .on_hover_text("Live bytes, counted by our own allocator");
            } else if cfg!(debug_assertions) {
                ui.label("Memory-tracking allocator not installed.");
            }
        }

        let mut is_tracking_callstacks = re_memory::accounting_allocator::is_tracking_callstacks();
        re_ui
            .checkbox(
                ui,
                &mut is_tracking_callstacks,
                "Detailed allocation tracking",
            )
            .on_hover_text("This will slow down the program");
        re_memory::accounting_allocator::set_tracking_callstacks(is_tracking_callstacks);

        if let Some(tracking_stats) = re_memory::accounting_allocator::tracking_stats() {
            ui.style_mut().wrap = Some(false);
            Self::tracking_stats(ui, tracking_stats);
        } else if !cfg!(target_arch = "wasm32") {
            ui.label(format!(
                "Set {RERUN_TRACK_ALLOCATIONS}=1 for detailed allocation tracking from startup."
            ));
        }
    }

    fn gpu_stats(ui: &mut egui::Ui, gpu_resource_stats: &WgpuResourcePoolStatistics) {
        egui::Grid::new("gpu resource grid")
            .num_columns(2)
            .show(ui, |ui| {
                let WgpuResourcePoolStatistics {
                    num_bind_group_layouts,
                    num_pipeline_layouts,
                    num_render_pipelines,
                    num_samplers,
                    num_shader_modules,
                    num_bind_groups,
                    num_buffers,
                    num_textures,
                    total_buffer_size_in_bytes,
                    total_texture_size_in_bytes,
                } = gpu_resource_stats;

                ui.label("# Bind Group Layouts:");
                ui.label(num_bind_group_layouts.to_string());
                ui.end_row();
                ui.label("# Pipeline Layouts:");
                ui.label(num_pipeline_layouts.to_string());
                ui.end_row();
                ui.label("# Render Pipelines:");
                ui.label(num_render_pipelines.to_string());
                ui.end_row();
                ui.label("# Samplers:");
                ui.label(num_samplers.to_string());
                ui.end_row();
                ui.label("# Shader Modules:");
                ui.label(num_shader_modules.to_string());
                ui.end_row();
                ui.label("# Bind Groups:");
                ui.label(num_bind_groups.to_string());
                ui.end_row();
                ui.label("# Buffers:");
                ui.label(num_buffers.to_string());
                ui.end_row();
                ui.label("# Textures:");
                ui.label(num_textures.to_string());
                ui.end_row();
                ui.label("Buffer Memory:");
                ui.label(re_format::format_bytes(*total_buffer_size_in_bytes as _));
                ui.end_row();
                ui.label("Texture Memory:");
                ui.label(re_format::format_bytes(*total_texture_size_in_bytes as _));
                ui.end_row();
            });
    }

    fn store_stats(
        ui: &mut egui::Ui,
        store_config: &DataStoreConfig,
        store_stats: &DataStoreStats,
    ) {
        egui::Grid::new("store config grid")
            .num_columns(3)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Limits").italics());
                ui.label("Row limit");
                ui.end_row();

                let label_rows = |ui: &mut egui::Ui, num_rows| {
                    if num_rows == u64::MAX {
                        ui.label("+∞")
                    } else {
                        ui.label(re_format::format_number(num_rows as _))
                    }
                };

                ui.label("Timeless:");
                label_rows(ui, u64::MAX);
                ui.end_row();

                ui.label("Temporal:");
                label_rows(ui, store_config.indexed_bucket_num_rows);
                ui.end_row();
            });

        ui.separator();

        egui::Grid::new("store stats grid")
            .num_columns(3)
            .show(ui, |ui| {
                let DataStoreStats {
                    type_registry,
                    metadata_registry,
                    autogenerated,
                    timeless,
                    temporal,
                    temporal_buckets,
                    total,
                } = *store_stats;

                ui.label(egui::RichText::new("Stats").italics());
                ui.label("Buckets");
                ui.label("Rows");
                ui.label("Size");
                ui.end_row();

                fn label_row_stats(ui: &mut egui::Ui, row_stats: DataStoreRowStats) {
                    let DataStoreRowStats {
                        num_rows,
                        num_bytes,
                    } = row_stats;

                    ui.label(re_format::format_number(num_rows as _));
                    ui.label(re_format::format_bytes(num_bytes as _));
                }

                ui.label("Type registry:");
                ui.label("");
                label_row_stats(ui, type_registry);
                ui.end_row();

                ui.label("Metadata registry:");
                ui.label("");
                label_row_stats(ui, metadata_registry);
                ui.end_row();

                ui.label("Cluster cache:");
                ui.label("");
                label_row_stats(ui, autogenerated);
                ui.end_row();

                ui.label("Timeless:");
                ui.label("");
                label_row_stats(ui, timeless);
                ui.end_row();

                ui.label("Temporal:");
                ui.label(re_format::format_number(temporal_buckets as _));
                label_row_stats(ui, temporal);
                ui.end_row();

                ui.label("Total");
                ui.label(re_format::format_number(temporal_buckets as _));
                label_row_stats(ui, total);
                ui.end_row();
            });
    }

    fn caches_stats(&self, ui: &mut egui::Ui, re_ui: &re_ui::ReUi, caches_stats: &CachesStats) {
        use std::sync::atomic::Ordering::Relaxed;

        let mut detailed_stats = self.prim_cache_detailed_stats.load(Relaxed);
        re_ui
            .checkbox(ui, &mut detailed_stats, "Detailed stats")
            .on_hover_text("Show detailed statistics when hovering entity paths below.\nThis will slow down the program.");
        self.prim_cache_detailed_stats
            .store(detailed_stats, Relaxed);

        let mut show_empty = self.prim_cache_show_empty.load(Relaxed);
        re_ui
            .checkbox(ui, &mut show_empty, "Show empty caches")
            .on_hover_text(
                "Show empty caches too.\nDangling buckets are generally the result of a bug.",
            );
        self.prim_cache_show_empty.store(show_empty, Relaxed);

        let CachesStats { latest_at, range } = caches_stats;

        if show_empty || !latest_at.is_empty() {
            ui.separator();
            ui.strong("LatestAt");
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .id_source("latest_at")
                .show(ui, |ui| {
                    egui::Grid::new("latest_at cache stats grid")
                        .num_columns(3)
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Entity").underline());
                            ui.label(egui::RichText::new("Rows").underline())
                                .on_hover_text(
                                    "How many distinct data timestamps have been cached?",
                                );
                            ui.label(egui::RichText::new("Size").underline());
                            ui.end_row();

                            for (entity_path, stats) in latest_at {
                                if !show_empty && stats.is_empty() {
                                    continue;
                                }

                                let res = ui.label(entity_path.to_string());
                                entity_stats_ui(ui, res, stats);
                                ui.end_row();
                            }
                        });
                });
        }

        if show_empty || !latest_at.is_empty() {
            ui.separator();
            ui.strong("Range");
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .id_source("range")
                .show(ui, |ui| {
                    egui::Grid::new("range cache stats grid")
                        .num_columns(4)
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Entity").underline());
                            ui.label(egui::RichText::new("Time range").underline());
                            ui.label(egui::RichText::new("Rows").underline())
                                .on_hover_text(
                                    "How many distinct data timestamps have been cached?",
                                );
                            ui.label(egui::RichText::new("Size").underline());
                            ui.end_row();

                            for (entity_path, stats_per_range) in range {
                                for (timeline, time_range, stats) in stats_per_range {
                                    if !show_empty && stats.is_empty() {
                                        continue;
                                    }

                                    let res = ui.label(entity_path.to_string());
                                    ui.label(format!(
                                        "{}({})",
                                        timeline.name(),
                                        timeline.format_time_range_utc(time_range)
                                    ));
                                    entity_stats_ui(ui, res, stats);
                                    ui.end_row();
                                }
                            }
                        });
                });
        }

        fn entity_stats_ui(
            ui: &mut egui::Ui,
            hover_response: egui::Response,
            entity_stats: &CachedEntityStats,
        ) {
            let CachedEntityStats {
                total_size_bytes,
                total_rows,
                per_component,
            } = entity_stats;

            if let Some(per_component) = per_component.as_ref() {
                hover_response.on_hover_ui_at_pointer(|ui| {
                    egui::Grid::new("component cache stats grid")
                        .num_columns(3)
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Component").underline());
                            ui.label(egui::RichText::new("Rows").underline());
                            ui.label(egui::RichText::new("Instances").underline());
                            ui.end_row();

                            for (component_name, stats) in per_component {
                                let &CachedComponentStats {
                                    total_rows,
                                    total_instances,
                                } = stats;

                                ui.label(component_name.to_string());
                                ui.label(re_format::format_number(total_rows as _));
                                ui.label(re_format::format_number(total_instances as _));
                                ui.end_row();
                            }
                        });
                });
            }

            ui.label(re_format::format_number(*total_rows as _));
            ui.label(re_format::format_bytes(*total_size_bytes as _));
        }
    }

    fn tracking_stats(
        ui: &mut egui::Ui,
        tracking_stats: re_memory::accounting_allocator::TrackingStatistics,
    ) {
        ui.label("counted = fully_tracked + stochastically_tracked + untracked + overhead");
        ui.label(format!(
            "fully_tracked: {} in {} allocs",
            format_bytes(tracking_stats.fully_tracked.size as _),
            format_number(tracking_stats.fully_tracked.count),
        ));
        ui.label(format!(
            "stochastically_tracked: {} in {} allocs",
            format_bytes(tracking_stats.stochastically_tracked.size as _),
            format_number(tracking_stats.stochastically_tracked.count),
        ));
        ui.label(format!(
            "untracked: {} in {} allocs (all smaller than {})",
            format_bytes(tracking_stats.untracked.size as _),
            format_number(tracking_stats.untracked.count),
            format_bytes(tracking_stats.track_size_threshold as _),
        ));
        ui.label(format!(
            "overhead: {} in {} allocs",
            format_bytes(tracking_stats.overhead.size as _),
            format_number(tracking_stats.overhead.count),
        ))
        .on_hover_text("Used for the book-keeping of the allocation tracker");

        egui::CollapsingHeader::new("Top memory consumers")
            .default_open(true)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        ui.set_min_width(750.0);
                        for callstack in tracking_stats.top_callstacks {
                            let stochastic_rate = callstack.stochastic_rate;
                            let is_stochastic = stochastic_rate > 1;

                            let text = format!(
                                "{}{} in {} allocs (≈{} / alloc){} - {}",
                                if is_stochastic { "≈" } else { "" },
                                format_bytes((callstack.extant.size * stochastic_rate) as _),
                                format_number(callstack.extant.count * stochastic_rate),
                                format_bytes(
                                    callstack.extant.size as f64 / callstack.extant.count as f64
                                ),
                                if stochastic_rate <= 1 {
                                    String::new()
                                } else {
                                    format!(" ({} stochastic samples)", callstack.extant.count)
                                },
                                summarize_callstack(&callstack.readable_backtrace.to_string())
                            );

                            if ui
                                .button(text)
                                .on_hover_text("Click to copy callstack to clipboard")
                                .clicked()
                            {
                                ui.output_mut(|o| {
                                    o.copied_text = callstack.readable_backtrace.to_string();
                                    if o.copied_text.is_empty() {
                                        // This is weird
                                        o.copied_text = "No callstack available".to_owned();
                                    }
                                });
                            }
                        }
                    });
            });
    }

    fn plot(&self, ui: &mut egui::Ui, limit: &MemoryLimit) {
        re_tracing::profile_function!();

        use itertools::Itertools as _;

        fn to_line(history: &egui::util::History<i64>) -> egui_plot::Line {
            egui_plot::Line::new(
                history
                    .iter()
                    .map(|(time, bytes)| [time, bytes as f64])
                    .collect_vec(),
            )
        }

        egui_plot::Plot::new("mem_history_plot")
            .min_size(egui::Vec2::splat(200.0))
            .label_formatter(|name, value| format!("{name}: {}", format_bytes(value.y)))
            .x_axis_formatter(|time, _, _| format!("{time} s"))
            .y_axis_formatter(|bytes, _, _| format_bytes(bytes))
            .show_x(false)
            .legend(egui_plot::Legend::default().position(egui_plot::Corner::LeftTop))
            .include_y(0.0)
            // TODO(emilk): turn off plot interaction, and always do auto-sizing
            .show(ui, |plot_ui| {
                if let Some(max_bytes) = limit.max_bytes {
                    plot_ui.hline(
                        egui_plot::HLine::new(max_bytes as f64)
                            .name("Limit (counted)")
                            .width(2.0),
                    );
                }

                for &time in &self.memory_purge_times {
                    plot_ui.vline(
                        egui_plot::VLine::new(time)
                            .name("RAM purge")
                            .color(egui::Color32::from_rgb(252, 161, 3))
                            .width(2.0),
                    );
                }

                let MemoryHistory {
                    resident,
                    counted,
                    counted_gpu,
                    counted_store,
                    counted_primary_caches,
                    counted_blueprint,
                } = &self.history;

                plot_ui.line(to_line(resident).name("Resident").width(1.5));
                plot_ui.line(to_line(counted).name("Counted").width(1.5));
                plot_ui.line(to_line(counted_gpu).name("Counted GPU").width(1.5));
                plot_ui.line(to_line(counted_store).name("Counted Store").width(1.5));
                plot_ui.line(
                    to_line(counted_primary_caches)
                        .name("Counted Primary Caches")
                        .width(1.5),
                );
                plot_ui.line(
                    to_line(counted_blueprint)
                        .name("Counted Blueprint")
                        .width(1.5),
                );
            });
    }
}

fn summarize_callstack(callstack: &str) -> String {
    let patterns = [
        ("App::receive_messages", "App::receive_messages"),
        ("w_store::store::ComponentBucket>::archive", "archive"),
        ("DataStore>::insert", "DataStore"),
        ("EntityDb", "EntityDb"),
        ("EntityDb", "EntityDb"),
        ("EntityTree", "EntityTree"),
        ("::LogMsg>::deserialize", "LogMsg"),
        ("::TimePoint>::deserialize", "TimePoint"),
        ("ImageCache", "ImageCache"),
        ("gltf", "gltf"),
        ("image::image", "image"),
        ("epaint::text::text_layout", "text_layout"),
        ("egui_wgpu", "egui_wgpu"),
        ("wgpu_hal", "wgpu_hal"),
        ("prepare_staging_buffer", "prepare_staging_buffer"),
        // -----
        // Very general:
        ("crossbeam::channel::Sender", "crossbeam::channel::Sender"),
        ("epaint::texture_atlas", "egui font texture"),
        (
            "alloc::collections::btree::map::BTreeSet<K,V,A>",
            "BTreeSet",
        ),
        (
            "alloc::collections::btree::map::BTreeMap<K,V,A>",
            "BTreeMap",
        ),
        ("std::collections::hash::map::HashMap<K,V,S>", "HashMap"),
    ];

    let mut all_summaries = vec![];

    for (pattern, summary) in patterns {
        if callstack.contains(pattern) {
            all_summaries.push(summary);
        }
    }

    all_summaries.join(", ")
}
