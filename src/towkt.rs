extern crate geo_types;

use types::Coord;
use types::CoordType;
use types::GeometryCollection;
use types::LineString;
use types::MultiLineString;
use types::MultiPoint;
use types::MultiPolygon;
use types::Point;
use types::Polygon;
use Geometry;
use Wkt;

/// A trait for converting values to WKT
pub trait ToWkt<T: CoordType> {
    /// Converts the value of `self` to an instance of WKT
    fn to_wkt(&self) -> Wkt<T>;
}

fn g_point_to_w_coord<T: CoordType>(g_point: &geo_types::Point<T>) -> Coord<T> {
    let geo_types::Point(coord) = *g_point;
    let geo_types::Coordinate { x, y } = coord;
    Coord {
        x: x,
        y: y,
        z: None,
        m: None,
    }
}

fn g_point_to_w_point<T: CoordType>(g_point: &geo_types::Point<T>) -> Point<T> {
    let coord = g_point_to_w_coord(g_point);
    Point(Some(coord))
}

fn g_points_to_w_coords<T: CoordType>(g_points: &Vec<geo_types::Point<T>>) -> Vec<Coord<T>> {
    let mut w_points = vec![];
    for g_point in g_points {
        w_points.push(g_point_to_w_coord(g_point));
    }
    w_points
}

fn g_linestring_to_w_linestring<T: CoordType>(g_linestring: &geo_types::LineString<T>) -> LineString<T> {
    let &geo_types::LineString(ref g_points) = g_linestring;
    g_points_to_w_linestring(g_points)
}

fn g_line_to_w_linestring<T: CoordType>(g_line: &geo_types::Line<T>) -> LineString<T> {
    g_points_to_w_linestring(&vec![g_line.start, g_line.end])
}

fn g_line_to_w_line<T: CoordType>(g_line: &geo_types::LineString<T>) -> LineString<T> {
    let &geo_types::LineString(ref g_points) = g_line;
    g_points_to_w_linestring(g_points)
}

fn g_points_to_w_linestring<T: CoordType>(g_points: &Vec<geo_types::Point<T>>) -> LineString<T> {
    let w_points = g_points_to_w_coords(g_points);
    LineString(w_points)
}

fn g_lines_to_w_lines<T: CoordType>(g_lines: &Vec<geo_types::LineString<T>>) -> Vec<LineString<T>> {
    let mut w_lines = vec![];
    for g_line in g_lines {
        let &geo_types::LineString(ref g_points) = g_line;
        w_lines.push(g_points_to_w_linestring(g_points));
    }
    w_lines
}

fn g_polygon_to_w_polygon<T: CoordType>(g_polygon: &geo_types::Polygon<T>) -> Polygon<T> {
    // let &geo_types::Polygon(ref outer_line, ref inner_lines) = g_polygon;
    let mut poly_lines = vec![];

    // Outer
    let &geo_types::LineString(ref outer_points) = &g_polygon.exterior;
    poly_lines.push(g_points_to_w_linestring(outer_points));

    // Inner
    let inner = g_lines_to_w_lines(&g_polygon.interiors);
    poly_lines.extend(inner.into_iter());

    Polygon(poly_lines)
}

fn g_mpoint_to_w_mpoint<T: CoordType>(g_mpoint: &geo_types::MultiPoint<T>) -> MultiPoint<T> {
    let &geo_types::MultiPoint(ref g_points) = g_mpoint;
    let w_coords = g_points_to_w_coords(g_points);
    let w_points = w_coords.into_iter().map(|c| Point(Some(c))).collect();
    MultiPoint(w_points)
}

fn g_mline_to_w_mline<T: CoordType>(g_mline: &geo_types::MultiLineString<T>) -> MultiLineString<T> {
    let &geo_types::MultiLineString(ref g_lines) = g_mline;
    let w_lines = g_lines_to_w_lines(g_lines);
    MultiLineString(w_lines)
}

fn g_polygons_to_w_polygons<T: CoordType>(g_polygons: &Vec<geo_types::Polygon<T>>) -> Vec<Polygon<T>> {
    let mut w_polygons = vec![];
    for g_polygon in g_polygons {
        w_polygons.push(g_polygon_to_w_polygon(g_polygon));
    }
    w_polygons
}

fn g_mpolygon_to_w_mpolygon<T: CoordType>(g_mpolygon: &geo_types::MultiPolygon<T>) -> MultiPolygon<T> {
    let &geo_types::MultiPolygon(ref g_polygons) = g_mpolygon;
    let w_polygons = g_polygons_to_w_polygons(g_polygons);
    MultiPolygon(w_polygons)
}

fn g_geocol_to_w_geocol<T: CoordType>(g_geocol: &geo_types::GeometryCollection<T>) -> GeometryCollection<T> {
    let &geo_types::GeometryCollection(ref g_geoms) = g_geocol;
    let mut w_geoms = vec![];
    for g_geom in g_geoms {
        let w_geom = g_geom_to_w_geom(g_geom);
        w_geoms.push(w_geom);
    }
    GeometryCollection(w_geoms)
}

fn g_geom_to_w_geom<T: CoordType>(g_geom: &geo_types::Geometry<T>) -> Geometry<T> {
    match g_geom {
        &geo_types::Geometry::Point(ref g_point) => g_point_to_w_point(g_point).as_item(),

        &geo_types::Geometry::Line(ref g_line) => g_line_to_w_linestring(g_line).as_item(),

        &geo_types::Geometry::LineString(ref g_line) => g_line_to_w_line(g_line).as_item(),

        &geo_types::Geometry::Polygon(ref g_polygon) => g_polygon_to_w_polygon(g_polygon).as_item(),

        &geo_types::Geometry::MultiPoint(ref g_mpoint) => g_mpoint_to_w_mpoint(g_mpoint).as_item(),

        &geo_types::Geometry::MultiLineString(ref g_mline) => g_mline_to_w_mline(g_mline).as_item(),

        &geo_types::Geometry::MultiPolygon(ref g_mpolygon) => {
            g_mpolygon_to_w_mpolygon(g_mpolygon).as_item()
        }

        &geo_types::Geometry::GeometryCollection(ref g_geocol) => {
            g_geocol_to_w_geocol(g_geocol).as_item()
        }
    }
}

impl<T: CoordType> ToWkt<T> for geo_types::Geometry<T> {
    fn to_wkt(&self) -> Wkt<T> {
        let w_geom: Geometry<T> = g_geom_to_w_geom(&self);
        Wkt {
            items: vec![w_geom],
        }
    }
}

#[cfg(test)]
mod tests {
    // use geo_types;
    #[test]
    fn converting_geo() {
        assert_eq!(1,2);
    }
}
