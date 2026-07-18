// Assumes d3 is globally available via d3_core.js

// Constructor wrapper - from flat array [x0, y0, x1, y1, ...]
export function delaunayFromFlat(points) {
  const float64Points = new Float64Array(points);
  return new d3.Delaunay(float64Points);
}

// Constructor wrapper - from point tuples [(x0, y0), (x1, y1), ...]
export function delaunayFromPoints(points) {
  // Convert array of tuples to flat array
  const flatPoints = [];
  for (const [x, y] of points) {
    flatPoints.push(x, y);
  }
  return new d3.Delaunay(new Float64Array(flatPoints));
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
  if (bounds) {
    return delaunayRef.voronoi(bounds);
  }
  return delaunayRef.voronoi();
}

// Rendering methods - return SVG path strings
export function delaunayRenderTriangles(delaunayRef) {
  return delaunayRef.render();
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
  return voronoiRef.render();
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
