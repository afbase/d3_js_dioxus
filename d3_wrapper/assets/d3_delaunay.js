// Assumes d3 is globally available via d3_core.js

// Constructor wrapper - from flat array [x0, y0, x1, y1, ...]
export function delaunayFromFlat(points) {
  const float64Points = new Float64Array(points);
  return new d3.Delaunay(float64Points);
}

// Constructor wrapper - from point tuples [(x0, y0), (x1, y1), ...]
export function delaunayFromPoints(points) {
  try {
    console.log(`[d3_delaunay] Creating Delaunay from ${points.length} points`);
    // Convert array of tuples to flat array
    const flatPoints = [];
    for (const [x, y] of points) {
      flatPoints.push(x, y);
    }
    const delaunay = new d3.Delaunay(new Float64Array(flatPoints));
    console.log(`[d3_delaunay] Successfully created Delaunay`);
    return delaunay;
  } catch (e) {
    console.error(`[d3_delaunay] Failed to create Delaunay:`, e);
    throw e;
  }
}

// Query methods - pass JsValue reference
export function delaunayFind(delaunayRef, x, y, startIndex = 0) {
  return delaunayRef.find(x, y, startIndex);
}

export function delaunayNeighbors(delaunayRef, i) {
  return Array.from(delaunayRef.neighbors(i));
}

// Voronoi generation
export function delaunayVoronoi(delaunayRef, bounds) {
  try {
    console.log(`[d3_delaunay] Creating Voronoi with bounds:`, bounds);
    const voronoi = bounds ? delaunayRef.voronoi(bounds) : delaunayRef.voronoi();
    console.log(`[d3_delaunay] Voronoi created successfully`);
    return voronoi;
  } catch (e) {
    console.error(`[d3_delaunay] Failed to create Voronoi:`, e);
    throw e;
  }
}

// Rendering methods - return SVG path strings
export function delaunayRenderTriangles(delaunayRef) {
  try {
    console.log(`[d3_delaunay] Rendering triangles`);
    const path = delaunayRef.render();
    console.log(`[d3_delaunay] Triangles rendered, path length: ${path.length}`);
    return path;
  } catch (e) {
    console.error(`[d3_delaunay] Failed to render triangles:`, e);
    throw e;
  }
}

export function delaunayRenderHull(delaunayRef) {
  return delaunayRef.renderHull();
}

export function delaunayRenderTriangle(delaunayRef, i) {
  return delaunayRef.renderTriangle(i);
}

export function delaunayRenderPoints(delaunayRef, context, r) {
  return delaunayRef.renderPoints(context, r);
}

// Property accessors
export function delaunayGetPoints(delaunayRef) {
  return Array.from(delaunayRef.points);
}

export function delaunayGetTriangles(delaunayRef) {
  return Array.from(delaunayRef.triangles);
}

export function delaunayGetHull(delaunayRef) {
  return Array.from(delaunayRef.hull);
}

export function delaunayGetHalfedges(delaunayRef) {
  return Array.from(delaunayRef.halfedges);
}

export function delaunayGetInedges(delaunayRef) {
  return Array.from(delaunayRef.inedges);
}

// Triangle methods
export function delaunayTrianglePolygon(delaunayRef, i) {
  return delaunayRef.trianglePolygon(i);
}

export function delaunayTrianglePolygons(delaunayRef) {
  return Array.from(delaunayRef.trianglePolygons());
}

// Voronoi methods
export function voronoiRender(voronoiRef) {
  try {
    console.log(`[d3_delaunay] Rendering Voronoi`);
    const path = voronoiRef.render();
    console.log(`[d3_delaunay] Voronoi rendered, path length: ${path.length}`);
    return path;
  } catch (e) {
    console.error(`[d3_delaunay] Failed to render Voronoi:`, e);
    throw e;
  }
}

export function voronoiRenderBounds(voronoiRef) {
  return voronoiRef.renderBounds();
}

export function voronoiRenderCell(voronoiRef, i) {
  return voronoiRef.renderCell(i);
}

export function voronoiContains(voronoiRef, i, x, y) {
  return voronoiRef.contains(i, x, y);
}

export function voronoiCellPolygon(voronoiRef, i) {
  return voronoiRef.cellPolygon(i);
}

export function voronoiCellPolygons(voronoiRef) {
  return Array.from(voronoiRef.cellPolygons());
}

// Voronoi property accessors
export function voronoiGetCircumcenters(voronoiRef) {
  return Array.from(voronoiRef.circumcenters);
}

export function voronoiGetVectors(voronoiRef) {
  return Array.from(voronoiRef.vectors);
}
