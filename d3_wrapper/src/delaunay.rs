use dioxus::prelude::{asset, Asset};
use dioxus_use_js::{use_js, JsValue};
use tracing::{debug, error, info};

// Import all wrapper functions from JS
// NOTE: Path is relative to d3_wrapper crate directory
use_js!("assets/d3_delaunay.js"::{
    delaunayFromFlat,
    delaunayFromPoints,
    delaunayFind,
    delaunayNeighbors,
    delaunayVoronoi,
    delaunayRenderTriangles,
    delaunayRenderHull,
    delaunayRenderTriangle,
    delaunayRenderPoints,
    delaunayGetPoints,
    delaunayGetTriangles,
    delaunayGetHull,
    delaunayGetHalfedges,
    delaunayGetInedges,
    delaunayTrianglePolygon,
    delaunayTrianglePolygons,
    voronoiRender,
});

/// A Delaunay triangulation computed from 2D points
///
/// # Example
/// ```no_run
/// use d3_wrapper::prelude::*;
///
/// let points = vec![(0.0, 0.0), (100.0, 0.0), (50.0, 100.0)];
/// let delaunay = Delaunay::from_points(points).await?;
/// let svg_path = delaunay.render_svg_path().await?;
/// ```
#[derive(Clone)]
pub struct Delaunay {
    inner: JsValue,
}

impl Delaunay {
    /// Create triangulation from flat array [x0, y0, x1, y1, ...]
    pub async fn from_flat_points(points: Vec<f64>) -> Result<Self, dioxus_use_js::JsError> {
        let inner = delaunayFromFlat(points).await?;
        Ok(Self { inner })
    }

    /// Create triangulation from point tuples [(x0, y0), (x1, y1), ...]
    pub async fn from_points(points: Vec<(f64, f64)>) -> Result<Self, dioxus_use_js::JsError> {
        debug!("Creating Delaunay from {} points", points.len());
        let inner = delaunayFromPoints(points).await?;
        info!("Successfully created Delaunay triangulation");
        Ok(Self { inner })
    }

    /// Find the closest point to (x, y)
    ///
    /// Returns the index of the nearest point in the points array.
    pub async fn find(&self, x: f64, y: f64) -> Result<usize, dioxus_use_js::JsError> {
        delaunayFind(self.inner.clone(), x, y, 0).await
    }

    /// Find the closest point to (x, y), starting search from a given point
    pub async fn find_from(
        &self,
        x: f64,
        y: f64,
        start_index: usize,
    ) -> Result<usize, dioxus_use_js::JsError> {
        delaunayFind(self.inner.clone(), x, y, start_index).await
    }

    /// Get neighboring point indices for point i
    pub async fn neighbors(&self, i: usize) -> Result<Vec<usize>, dioxus_use_js::JsError> {
        delaunayNeighbors(self.inner.clone(), i).await
    }

    /// Create Voronoi diagram with optional bounds [xmin, ymin, xmax, ymax]
    ///
    /// If bounds are not provided, uses the bounding box of the points.
    pub async fn voronoi(
        &self,
        bounds: Option<[f64; 4]>,
    ) -> Result<Voronoi, dioxus_use_js::JsError> {
        debug!("Creating Voronoi diagram with bounds: {:?}", bounds);
        match delaunayVoronoi(self.inner.clone(), bounds).await {
            Ok(inner) => {
                info!("Successfully created Voronoi diagram");
                Ok(Voronoi { inner })
            }
            Err(e) => {
                error!("Failed to create Voronoi diagram: {:?}", e);
                Err(e)
            }
        }
    }

    /// Render all triangulation edges as SVG path
    pub async fn render_svg_path(&self) -> Result<String, dioxus_use_js::JsError> {
        debug!("Rendering triangulation as SVG path");
        match delaunayRenderTriangles(self.inner.clone()).await {
            Ok(path) => {
                info!("Successfully rendered triangulation path (length: {:?})", path);
                Ok(path)
            }
            Err(e) => {
                error!("Failed to render triangulation: {:?}", e);
                Err(e)
            }
        }
    }

    /// Render convex hull as SVG path
    pub async fn render_hull_svg_path(&self) -> Result<String, dioxus_use_js::JsError> {
        delaunayRenderHull(self.inner.clone()).await
    }

    /// Render single triangle as SVG path
    pub async fn render_triangle_svg_path(
        &self,
        i: usize,
    ) -> Result<String, dioxus_use_js::JsError> {
        delaunayRenderTriangle(self.inner.clone(), i).await
    }

    /// Get flat points array [x0, y0, x1, y1, ...]
    pub async fn get_points(&self) -> Result<Vec<f64>, dioxus_use_js::JsError> {
        delaunayGetPoints(self.inner.clone()).await
    }

    /// Get triangle indices [i0, j0, k0, i1, j1, k1, ...]
    ///
    /// Each consecutive triplet of indices represents one triangle.
    pub async fn get_triangles(&self) -> Result<Vec<u32>, dioxus_use_js::JsError> {
        delaunayGetTriangles(self.inner.clone()).await
    }

    /// Get convex hull point indices
    pub async fn get_hull(&self) -> Result<Vec<u32>, dioxus_use_js::JsError> {
        delaunayGetHull(self.inner.clone()).await
    }

    /// Get halfedge data structure
    pub async fn get_halfedges(&self) -> Result<Vec<i32>, dioxus_use_js::JsError> {
        delaunayGetHalfedges(self.inner.clone()).await
    }

    /// Get inedges (incoming halfedge for each point)
    pub async fn get_inedges(&self) -> Result<Vec<i32>, dioxus_use_js::JsError> {
        delaunayGetInedges(self.inner.clone()).await
    }

    /// Get polygon for triangle i as array of [x, y] points
    pub async fn triangle_polygon(
        &self,
        i: usize,
    ) -> Result<Vec<[f64; 2]>, dioxus_use_js::JsError> {
        delaunayTrianglePolygon(self.inner.clone(), i).await
    }

    /// Get all triangle polygons
    pub async fn triangle_polygons(&self) -> Result<Vec<Vec<[f64; 2]>>, dioxus_use_js::JsError> {
        delaunayTrianglePolygons(self.inner.clone()).await
    }
}

/// A Voronoi diagram (dual of Delaunay triangulation)
///
/// # Example
/// ```no_run
/// use d3_wrapper::prelude::*;
///
/// let points = vec![(0.0, 0.0), (100.0, 0.0), (50.0, 100.0)];
/// let delaunay = Delaunay::from_points(points).await?;
/// let voronoi = delaunay.voronoi(Some([0.0, 0.0, 100.0, 100.0])).await?;
/// let svg_path = voronoi.render_svg_path().await?;
/// ```
#[derive(Clone)]
pub struct Voronoi {
    inner: JsValue,
}

impl Voronoi {
    /// Render all cells as SVG path
    pub async fn render_svg_path(&self) -> Result<String, dioxus_use_js::JsError> {
        debug!("Rendering Voronoi as SVG path");
        match voronoiRender(self.inner.clone()).await {
            Ok(path) => {
                info!("Successfully rendered Voronoi path (length: {:?})", path);
                Ok(path)
            }
            Err(e) => {
                error!("Failed to render Voronoi: {:?}", e);
                Err(e)
            }
        }
    }

    // Note: Additional Voronoi methods (renderBounds, renderCell, contains, cellPolygon, etc.)
    // are available in the JavaScript wrapper but not yet exposed in this Rust API.
    // They can be added by including them in the use_js! macro invocation above.
}
