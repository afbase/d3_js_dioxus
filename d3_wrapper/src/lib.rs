//! Type-safe Rust bindings to d3.js for Dioxus applications.
//!
//! This library provides type-safe wrappers around d3.js functionality using the
//! `dioxus-use-js` crate. It loads d3.js from CDN and provides ergonomic Rust APIs
//! for d3's various modules.
//!
//! # Getting Started
//!
//! Before using any d3 functionality, you must initialize d3.js:
//!
//! ```no_run
//! use d3_wrapper::prelude::*;
//! use dioxus::prelude::*;
//!
//! #[component]
//! fn App() -> Element {
//!     let d3_ready = use_resource(|| async move {
//!         init_d3().await
//!     });
//!
//!     match d3_ready.read().as_ref() {
//!         Some(Ok(_)) => rsx! { MyVisualization {} },
//!         Some(Err(e)) => rsx! { div { "Failed to load d3.js: {e}" } },
//!         None => rsx! { div { "Loading d3.js..." } }
//!     }
//! }
//! ```
//!
//! # Example: Delaunay Triangulation
//!
//! ```no_run
//! use d3_wrapper::prelude::*;
//!
//! async fn create_triangulation() -> Result<Delaunay, dioxus_use_js::JsError> {
//!     let points = vec![
//!         (0.0, 0.0),
//!         (100.0, 0.0),
//!         (50.0, 100.0),
//!     ];
//!
//!     Delaunay::from_points(points).await
//! }
//! ```
//!
//! # Module Coverage
//!
//! Currently implemented:
//! - **d3-delaunay**: Delaunay triangulation and Voronoi diagrams
//!
//! More d3 modules can be added following the same pattern established here.

pub mod core;
pub mod delaunay;

/// Convenience re-exports for common usage
pub mod prelude {
    pub use crate::core::*;
    pub use crate::delaunay::*;
}
