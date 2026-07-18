# D3.js Dioxus Wrapper

A type-safe Rust wrapper over d3.js for Dioxus applications using the `dioxus-use-js` crate.

## Project Structure

```
d3_js_dioxus/
├── d3_wrapper/          # Library crate - type-safe d3.js bindings
│   ├── src/
│   │   ├── lib.rs      # Library entry point
│   │   ├── core.rs     # CDN loading functionality
│   │   └── delaunay.rs # d3-delaunay API wrapper
│   └── assets/
│       ├── d3_core.js      # JavaScript CDN loader
│       └── d3_delaunay.js  # Delaunay wrapper functions
└── example/            # Example application
    ├── src/
    │   └── main.rs    # Interactive Voronoi/Delaunay demo
    └── style.css      # Demo styling
```

## Features

- **Type-safe API**: Rust wrappers around d3.js functions with compile-time checks
- **CDN loading**: Automatically loads d3.js v7.9.0 from jsdelivr CDN
- **d3-delaunay support**: Complete wrapper for Delaunay triangulation and Voronoi diagrams
- **Cross-platform**: Works on Web, Desktop, and Mobile via Dioxus
- **Extensible**: Easy to add more d3 modules following the established patterns

## Building

```bash
# Check the workspace
cargo check --workspace

# Run the example (once dx CLI is configured)
cd example
dx serve
```

## Library Usage

```rust
use d3_wrapper::prelude::*;
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    // Initialize d3.js first
    let d3_ready = use_resource(|| async move {
        init_d3().await
    });

    match d3_ready.read().as_ref() {
        Some(Ok(_)) => rsx! { MyVisualization {} },
        Some(Err(e)) => rsx! { div { "Failed to load d3.js: {e}" } },
        None => rsx! { div { "Loading d3.js..." } }
    }
}

#[component]
fn MyVisualization() -> Element {
    let points = vec![(0.0, 0.0), (100.0, 0.0), (50.0, 100.0)];

    let delaunay = use_resource(move || async move {
        Delaunay::from_points(points).await
    });

    // Use delaunay in your visualization
    rsx! {
        // ... render SVG paths, etc.
    }
}
```

## Example Application

The example demonstrates an interactive Voronoi/Delaunay visualization with:
- Random point generation (50 points)
- Delaunay triangulation rendering
- Voronoi diagram rendering
- View mode toggle (triangulation/voronoi/both)
- Click to find nearest point
- Regenerate points button

## API Documentation

### Core Module

- `init_d3()` - Load d3.js from CDN
- `is_d3_loaded()` - Check if d3.js is already loaded

### Delaunay Module

#### Delaunay Struct
- `from_flat_points(points: Vec<f64>)` - Create from [x0, y0, x1, y1, ...]
- `from_points(points: Vec<(f64, f64)>)` - Create from point tuples
- `find(x, y)` - Find nearest point index
- `neighbors(i)` - Get neighboring point indices
- `voronoi(bounds)` - Create Voronoi diagram
- `render_svg_path()` - Render triangulation as SVG path
- `render_hull_svg_path()` - Render convex hull as SVG path
- `get_triangles()` - Get triangle indices
- `get_hull()` - Get convex hull indices
- And more...

#### Voronoi Struct
- `render_svg_path()` - Render all Voronoi cells as SVG path

## Technical Details

- **Dioxus version**: 0.7
- **dioxus-use-js version**: 0.4
- **D3.js version**: 7.9.0 (loaded from CDN)

The wrapper uses `JsValue` to store opaque JavaScript references, avoiding serialization overhead for d3 objects.

## Extending the Library

To add more d3 modules:

1. Create a JavaScript wrapper file in `d3_wrapper/assets/` with exported functions
2. Create a Rust module in `d3_wrapper/src/` that uses `use_js!` to import those functions
3. Add ergonomic Rust wrapper types and methods
4. Export the module from `lib.rs`

Follow the patterns established in `delaunay.rs` for consistency.

## Notes

- JavaScript files must be in the `d3_wrapper/assets/` directory for the asset system to find them
- The `use_js!` macro requires both `asset` and `Asset` to be in scope from `dioxus::prelude`
- Functions that return values need explicit type annotations to help Rust's type inference
- Additional Voronoi methods are available in the JavaScript wrapper but not all are exposed yet in the Rust API

## License

MIT OR Apache-2.0 (same as d3_wrapper crate)
