use d3_wrapper::prelude::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error, info};

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    // Initialize d3 on mount
    let d3_ready = use_resource(|| async move { init_d3().await });

    match d3_ready.read().as_ref() {
        Some(Ok(_)) => rsx! { VoronoiDemo {} },
        Some(Err(e)) => rsx! {
            div { class: "error",
                h1 { "Failed to load d3.js" }
                p { "{e}" }
            }
        },
        None => rsx! {
            div { class: "loading",
                h1 { "Loading d3.js..." }
            }
        },
    }
}

#[component]
fn VoronoiDemo() -> Element {
    // State
    let mut points = use_signal(|| generate_random_points(50, 800.0, 600.0));
    let mut view_mode = use_signal(|| ViewMode::Both);
    let mut selected_point = use_signal(|| None::<usize>);

    // Compute triangulation reactively
    let delaunay_resource = use_resource(move || {
        let pts = points.read().clone();
        async move {
            info!("Computing Delaunay triangulation for {} points", pts.len());
            match Delaunay::from_points(pts).await {
                Ok(d) => {
                    info!("Delaunay triangulation computed successfully");
                    Ok(d)
                }
                Err(e) => {
                    error!("Delaunay triangulation failed: {:?}", e);
                    Err(e)
                }
            }
        }
    });

    // Render paths
    let render_data = use_resource(move || {
        let delaunay_opt = delaunay_resource.read().as_ref().and_then(|r| r.as_ref().ok().cloned());
        let mode = *view_mode.read();

        async move {
            info!("Rendering visualization data, mode: {:?}", mode);

            match delaunay_opt {
                Some(delaunay) => {
                    debug!("Delaunay available, rendering paths");

                    let triangulation = if matches!(mode, ViewMode::Triangulation | ViewMode::Both) {
                        match delaunay.render_svg_path().await {
                            Ok(path) => {
                                info!("Triangulation path rendered successfully");
                                Some(path)
                            }
                            Err(e) => {
                                error!("Triangulation rendering failed: {:?}", e);
                                None
                            }
                        }
                    } else {
                        None
                    };

                    let voronoi = if matches!(mode, ViewMode::Voronoi | ViewMode::Both) {
                        match delaunay.voronoi(Some([0.0, 0.0, 800.0, 600.0])).await {
                            Ok(vor) => {
                                match vor.render_svg_path().await {
                                    Ok(path) => {
                                        info!("Voronoi path rendered successfully");
                                        Some(path)
                                    }
                                    Err(e) => {
                                        error!("Voronoi path rendering failed: {:?}", e);
                                        None
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Voronoi creation failed: {:?}", e);
                                None
                            }
                        }
                    } else {
                        None
                    };

                    Some(RenderData {
                        triangulation,
                        voronoi,
                    })
                }
                None => {
                    error!("Delaunay triangulation not available for rendering");
                    None
                }
            }
        }
    });

    // Event handlers
    let handle_click = move |evt: Event<MouseData>| {
        spawn(async move {
            let coords = evt.page_coordinates();
            let (x, y) = (coords.x, coords.y);

            if let Some(Ok(delaunay)) = delaunay_resource.read().as_ref() {
                if let Ok(idx) = delaunay.find(x, y).await {
                    selected_point.set(Some(idx));
                }
            }
        });
    };

    let regenerate_points = move |_| {
        points.set(generate_random_points(50, 800.0, 600.0));
        selected_point.set(None);
    };

    // UI
    rsx! {
        style { {include_str!("../style.css")} }

        div { class: "container",
            h1 { "D3 Delaunay/Voronoi Demo" }

            div { class: "controls",
                button { onclick: regenerate_points, "Regenerate Points" }
                button {
                    onclick: move |_| view_mode.set(ViewMode::Triangulation),
                    class: if matches!(*view_mode.read(), ViewMode::Triangulation) { "active" } else { "" },
                    "Triangulation"
                }
                button {
                    onclick: move |_| view_mode.set(ViewMode::Voronoi),
                    class: if matches!(*view_mode.read(), ViewMode::Voronoi) { "active" } else { "" },
                    "Voronoi"
                }
                button {
                    onclick: move |_| view_mode.set(ViewMode::Both),
                    class: if matches!(*view_mode.read(), ViewMode::Both) { "active" } else { "" },
                    "Both"
                }
            }

            svg {
                width: "800",
                height: "600",
                class: "visualization",
                onclick: handle_click,

                match render_data.read().as_ref() {
                    Some(Some(data)) => rsx! {
                        if let Some(tri) = &data.triangulation {
                            path { d: "{tri}", stroke: "#888", fill: "none", stroke_width: "1" }
                        }
                        if let Some(vor) = &data.voronoi {
                            path { d: "{vor}", stroke: "#0066cc", fill: "none", stroke_width: "1.5" }
                        }

                        for (i, (x, y)) in points.read().iter().enumerate() {
                            circle {
                                cx: "{x}",
                                cy: "{y}",
                                r: if selected_point.read().map_or(false, |s| s == i) { "6" } else { "3" },
                                fill: if selected_point.read().map_or(false, |s| s == i) { "#ff0000" } else { "#000000" },
                                stroke: if selected_point.read().map_or(false, |s| s == i) { "#ffffff" } else { "none" },
                                stroke_width: if selected_point.read().map_or(false, |s| s == i) { "2" } else { "0" }
                            }
                        }
                    },
                    Some(None) => rsx! {
                        text { x: "400", y: "300", text_anchor: "middle", "Error computing triangulation" }
                    },
                    None => rsx! {
                        text { x: "400", y: "300", text_anchor: "middle", "Computing..." }
                    }
                }
            }

            if let Some(idx) = *selected_point.read() {
                if let Some(points_vec) = points.read().get(idx) {
                    div { class: "info",
                        "Selected point: {idx} at ({points_vec.0:.1}, {points_vec.1:.1})"
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ViewMode {
    Triangulation,
    Voronoi,
    Both,
}

struct RenderData {
    triangulation: Option<String>,
    voronoi: Option<String>,
}

fn generate_random_points(count: usize, width: f64, height: f64) -> Vec<(f64, f64)> {
    use std::collections::HashSet;

    let mut points = Vec::new();
    let mut seen = HashSet::new();

    // Use getrandom for WASM-compatible random seed generation
    let mut buf = [0u8; 8];
    getrandom::fill(&mut buf).unwrap();
    let mut rng_state = u64::from_le_bytes(buf);

    while points.len() < count {
        // Simple LCG (Linear Congruential Generator) for random numbers
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let x = ((rng_state / 65536) % (width as u64)) as f64;

        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let y = ((rng_state / 65536) % (height as u64)) as f64;

        let key = (x as i32, y as i32);

        if seen.insert(key) {
            points.push((x, y));
        }
    }

    points
}
