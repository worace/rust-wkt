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
// use Wkt;

// trait CoordCommon = geo_types::CoordinateType + CoordType;

// Tried to implement std::convert::From for this, but ran into issues with
// https://github.com/rust-lang/rust/issues/24745 so just doing it as a new trait ATM
pub trait ToGeo<T: geo_types::CoordinateType> {
    fn to_geo(&self) -> Result<geo_types::Geometry<T>, &'static str>;
}

fn coord_to_g_coord<T: geo_types::CoordinateType + CoordType>(
    coord: &Coord<T>,
) -> geo_types::Coordinate<T> {
    geo_types::Coordinate {
        x: coord.x,
        y: coord.y,
    }
}

fn linestring_to_g_linestring<T: geo_types::CoordinateType + CoordType>(
    ls: &LineString<T>,
) -> geo_types::LineString<T> {
    let mut g_coords: Vec<geo_types::Coordinate<T>> = vec![];
    for c in &ls.0 {
        g_coords.push(coord_to_g_coord(c));
    }
    g_coords.into_iter().collect()
}

fn polygon_to_g_polygon<T: geo_types::CoordinateType + CoordType>(
    poly: &Polygon<T>,
) -> geo_types::Polygon<T> {
    if poly.0.is_empty() {
        geo_types::Polygon::new(geo_types::LineString(vec![]), vec![])
    } else {
        let mut g_rings: Vec<geo_types::LineString<T>> = vec![];
        for r in &poly.0 {
            g_rings.push(linestring_to_g_linestring(r));
        }

        let exterior = g_rings.remove(0);
        geo_types::Polygon {
            exterior: exterior,
            interiors: g_rings,
        }
    }
}

fn point_to_g_point<T: geo_types::CoordinateType + CoordType>(point: &Point<T>) -> Option<geo_types::Point<T>> {
    if let Some(coord) = &point.0 {
        Some(geo_types::Point(coord_to_g_coord(coord)))
    } else {
        None
    }
}

impl<T: geo_types::CoordinateType + CoordType> ToGeo<T> for Geometry<T> {
    fn to_geo(self: &Geometry<T>) -> Result<geo_types::Geometry<T>, &'static str> {
        match *self {
            Geometry::Point(Point(Some(ref c))) => {
                let g_coord = coord_to_g_coord(c);
                let g_point = geo_types::Point(g_coord);
                Ok(geo_types::Geometry::Point(g_point))
            }
            Geometry::LineString(ref ls) => Ok(geo_types::Geometry::LineString(
                linestring_to_g_linestring(ls),
            )),
            Geometry::Polygon(ref poly) => {
                Ok(geo_types::Geometry::Polygon(polygon_to_g_polygon(poly)))
            }
            Geometry::MultiPoint(MultiPoint(ref points)) => {
                let g_points = points
                    .iter()
                    .map(|p| point_to_g_point(p))
                    .filter(|p| p.is_some())
                    .map(|p| p.unwrap())
                    .collect();
                let g_mp = geo_types::MultiPoint(g_points);
                Ok(geo_types::Geometry::MultiPoint(g_mp))
            }
            Geometry::MultiLineString(MultiLineString(ref lines)) => {
                let g_lines = lines
                    .iter()
                    .map(|l| linestring_to_g_linestring(l))
                    .collect();
                let g_mls = geo_types::MultiLineString(g_lines);
                Ok(geo_types::Geometry::MultiLineString(g_mls))
            }
            Geometry::MultiPolygon(MultiPolygon(ref polygons)) => {
                let g_polys = polygons
                    .iter()
                    .map(|p| polygon_to_g_polygon(p))
                    .collect();
                let g_mp = geo_types::MultiPolygon(g_polys);
                Ok(geo_types::Geometry::MultiPolygon(g_mp))
            }
            Geometry::GeometryCollection(GeometryCollection(ref geoms)) => {
                let mut g_geoms: Vec<geo_types::Geometry<T>> = vec![];
                for geom in geoms {
                    let g_geom = try!(geom.to_geo());
                    g_geoms.push(g_geom);
                }
                let g_gc = geo_types::GeometryCollection(g_geoms);
                Ok(geo_types::Geometry::GeometryCollection(g_gc))
            }
            _ => Err("not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate geo_types;
    use togeo::ToGeo;
    use types::Coord;
    use types::Point;
    use Geometry;
    use Wkt;
    // use conversion::From;
    // use self::From;
    // use self::geo_types;

    #[test]
    fn converting_point_to_geo() {
        let c: Coord<f64> = Coord::new(12.0, 34.0);
        let p = Geometry::Point(Point(Some(c)));
        let g_point: geo_types::Point<f64> = p.to_geo().unwrap().as_point().unwrap();
        assert_eq!(12.0, g_point.0.x);
        assert_eq!(34.0, g_point.0.y);
        // let poly: geo_types::Geometry::Polygon(geo_types::Polygon<f64>) = geo_types::Polygon::new(coords.into(), vec![]);
    }

    #[test]
    fn converting_linestring_to_geo() {
        let wkt: Wkt<f64> = Wkt::from_str("LINESTRING (10 -20, -0 -0.5)").ok().unwrap();
        let ls = &wkt.items[0];
        assert_eq!(1, wkt.items.len());
        match ls {
            Geometry::LineString(_) => {
                let g_ls = ls.to_geo().unwrap().as_linestring().unwrap();

                let mut coords = g_ls.into_iter();

                let c1 = coords.next().unwrap();
                assert_eq!(10.0, c1.x());
                assert_eq!(-20.0, c1.y());

                let c2 = coords.next().unwrap();
                assert_eq!(0.0, c2.x());
                assert_eq!(-0.5, c2.y());
            }
            _ => assert!(false, "should be linestring!"),
        }
    }

    #[test]
    fn converting_empty_poly() {
        let wkt: Wkt<f64> = Wkt::from_str("POLYGON EMPTY").ok().unwrap();
        assert_eq!(1, wkt.items.len());
        let poly = &wkt.items[0];
        match poly {
            Geometry::Polygon(_) => {
                let g_poly: geo_types::Polygon<f64> = poly.to_geo().unwrap().as_polygon().unwrap();
                assert_eq!(0, g_poly.exterior.into_iter().len());
                assert_eq!(0, g_poly.interiors.len());
            }
            _ => assert!(false, "should be polygon!"),
        }
    }

    #[test]
    fn converting_polygon_to_geo() {
        let wkt: Wkt<f64> = Wkt::from_str("POLYGON ((8 4, 4 0, 0 4, 8 4), (7 3, 4 1, 1 4, 7 3))")
            .ok()
            .unwrap();
        assert_eq!(1, wkt.items.len());
        let poly = &wkt.items[0];
        match poly {
            Geometry::Polygon(_) => {
                let g_poly: geo_types::Polygon<f64> = poly.to_geo().unwrap().as_polygon().unwrap();

                let outer_exp = vec![(8.0, 4.0), (4.0, 0.0), (0.0, 4.0), (8.0, 4.0)];
                let outer_coords = g_poly.exterior.into_iter().map(|p| p.0).map(|c| (c.x, c.y));

                for ((x1, y1), (x2, y2)) in outer_coords.zip(outer_exp) {
                    assert_eq!(x1, x2);
                    assert_eq!(y1, y2);
                }

                assert_eq!(1, g_poly.interiors.len());

                let inner_exp = vec![(7.0, 3.0), (4.0, 1.0), (1.0, 4.0), (7.0, 3.0)];
                let inner_coords = g_poly
                    .interiors
                    .clone()
                    .pop()
                    .unwrap()
                    .into_iter()
                    .map(|p| p.0)
                    .map(|c| (c.x, c.y));

                for ((x1, y1), (x2, y2)) in inner_coords.zip(inner_exp) {
                    assert_eq!(x1, x2);
                    assert_eq!(y1, y2);
                }
            }
            _ => assert!(false, "should be polygon!"),
        }
    }

    #[test]
    fn converting_empty_multi_point() {
        let wkt: Wkt<f64> = Wkt::from_str("MULTIPOINT EMPTY").ok().unwrap();
        let mp = &wkt.items[0];
        assert_eq!(1, wkt.items.len());
        match mp {
            Geometry::MultiPoint(_) => {
                let g_mp: geo_types::MultiPoint<f64> = mp.to_geo().unwrap().as_multipoint().unwrap();
                assert_eq!(0, g_mp.0.len());
            }
            _ => assert!(false, "Should be a MultiPoint"),
        }
    }

    #[test]
    fn converting_multi_point_to_geo() {
        let wkt: Wkt<f64> = Wkt::from_str("MULTIPOINT ((8 4), (4 0))").ok().unwrap();
        let mp = &wkt.items[0];
        match mp {
            Geometry::MultiPoint(_) => {
                let g_mp: geo_types::MultiPoint<f64> = mp.to_geo().unwrap().as_multipoint().unwrap();
                let exp = vec![(8.0, 4.0), (4.0, 0.0)];
                let coords = g_mp.into_iter().map(|p| p.0).map(|c| (c.x, c.y));

                assert_eq!(exp.len(), coords.len());

                for ((x1, y1), (x2, y2)) in coords.zip(exp) {
                    assert_eq!(x1, x2);
                    assert_eq!(y1, y2);
                }
            }
            _ => assert!(false, "Should be a MultiPoint"),
        }
    }

    #[test]
    fn converting_empty_multi_line_string() {
        let wkt: Wkt<f64> = Wkt::from_str("MULTILINESTRING EMPTY")
            .ok()
            .unwrap();
        assert_eq!(1, wkt.items.len());
        let mls = &wkt.items[0];
        match mls {
            Geometry::MultiLineString(_) => {
                let g_mls: geo_types::MultiLineString<f64> = mls.to_geo().unwrap().as_multilinestring().unwrap();
                assert_eq!(0, g_mls.0.len());
            }
            _ => assert!(false, "Should be a MultiLineString"),
        }
    }

    #[test]
    fn converting_multi_line_string() {
        let wkt: Wkt<f64> = Wkt::from_str("MULTILINESTRING ((8 4, -3 0), (4 0, 6 -10))")
            .ok()
            .unwrap();
        assert_eq!(1, wkt.items.len());
        let mls = &wkt.items[0];
        match mls {
            Geometry::MultiLineString(_) => {
                let g_mls: geo_types::MultiLineString<f64> = mls.to_geo().unwrap().as_multilinestring().unwrap();
                assert_eq!(2, g_mls.0.len());

                let exp = vec![
                    vec![(8.0, 4.0), (-3.0, 0.0)],
                    vec![(4.0, 0.0), (6.0, -10.0)]
                ];

                for (i, ls) in g_mls.into_iter().enumerate() {
                    let coords_i = ls.into_iter().map(|p| p.0).map(|c| (c.x, c.y));
                    let exp_i = exp.get(i).unwrap();

                    for ((x1, y1), (x2, y2)) in coords_i.zip(exp_i.iter()) {
                        assert_eq!(x1, *x2);
                        assert_eq!(y1, *y2);
                    }
                }
            }
            _ => assert!(false, "Should be a MultiLineString"),
        }
    }

    #[test]
    fn converting_empty_multi_poly() {
        let wkt: Wkt<f64> = Wkt::from_str("MULTIPOLYGON EMPTY")
            .ok()
            .unwrap();
        assert_eq!(1, wkt.items.len());
        let mp = &wkt.items[0];
        match mp {
            Geometry::MultiPolygon(_) => {
                let g_mls: geo_types::MultiPolygon<f64> = mp.to_geo().unwrap().as_multipolygon().unwrap();
                assert_eq!(0, g_mls.0.len());
            }
            _ => assert!(false, "Should be a MultiLineString"),
        }
    }

    #[test]
    fn converting_multi_polygon() {
        let wkt_str = "MULTIPOLYGON (((40 40, 20 45, 45 30, 40 40)),
                                     ((20 35, 10 30, 10 10, 30 5, 45 20, 20 35),
                                      (30 20, 20 15, 20 25, 30 20)))";
        let wkt: Wkt<f64> = Wkt::from_str(wkt_str)
            .ok()
            .unwrap();
        assert_eq!(1, wkt.items.len());
        let mp = &wkt.items[0];
        match mp {
            Geometry::MultiPolygon(_) => {
                let g_mls: geo_types::MultiPolygon<f64> = mp.to_geo().unwrap().as_multipolygon().unwrap();
                assert_eq!(2, g_mls.0.len());

                let exp_outer1 = vec![(40.0, 40.0), (20.0, 45.0), (45.0, 30.0), (40.0, 40.0)];

                let poly1 = &g_mls.0[0];
                let outer_coords = poly1.exterior.clone().into_iter().map(|p| p.0).map(|c| (c.x, c.y));

                for ((x1, y1), (x2, y2)) in outer_coords.zip(exp_outer1.iter()) {
                    assert_eq!(x1, *x2);
                    assert_eq!(y1, *y2);
                }
                assert_eq!(0, poly1.interiors.len());

                let poly2 = &g_mls.0[1];
                let outer_coords = poly2.exterior.clone().into_iter().map(|p| p.0).map(|c| (c.x, c.y));
                let inner_coords = poly2.interiors[0].clone().into_iter().map(|p| p.0).map(|c| (c.x, c.y));

                let exp_outer2 = vec![(20.0, 35.0), (10.0, 30.0), (10.0, 10.0),
                                      (30.0, 5.0), (45.0, 20.0), (20.0, 35.0)];

                for ((x1, y1), (x2, y2)) in outer_coords.zip(exp_outer2.iter()) {
                    assert_eq!(x1, *x2);
                    assert_eq!(y1, *y2);
                }

                assert_eq!(1, poly2.interiors.len());
                let exp_inner = vec![(30.0, 20.0), (20.0, 15.0), (20.0, 25.0), (30.0, 20.0)];
                for ((x1, y1), (x2, y2)) in inner_coords.zip(exp_inner.iter()) {
                    assert_eq!(x1, *x2);
                    assert_eq!(y1, *y2);
                }
            }
            _ => assert!(false, "Should be a MultiPolygon"),
        }
    }

    #[test]
    fn converting_empty_geometry_collection() {
        let wkt: Wkt<f64> = Wkt::from_str("GEOMETRYCOLLECTION EMPTY")
            .ok()
            .unwrap();
        assert_eq!(1, wkt.items.len());
        let gc = &wkt.items[0];
        match gc {
            Geometry::GeometryCollection(_) => {
                let geom: geo_types::Geometry<f64> = gc.to_geo().unwrap();
                match geom {
                    geo_types::Geometry::GeometryCollection(g_gc) => {
                        assert_eq!(0, g_gc.0.len());
                    }
                    _ => assert!(false, "Should be a Geom Collection")
                }
            }
            _ => assert!(false, "Should be a Geom Collection"),
        }
    }

    #[test]
    fn converting_geometry_collection() {
        let wkt: Wkt<f64> = Wkt::from_str("GEOMETRYCOLLECTION(POINT(4 6),LINESTRING(4 6,7 10))")
            .ok()
            .unwrap();
        assert_eq!(1, wkt.items.len());
        let gc = &wkt.items[0];
        match gc {
            Geometry::GeometryCollection(_) => {
                let geom: geo_types::Geometry<f64> = gc.to_geo().unwrap();
                match geom {
                    geo_types::Geometry::GeometryCollection(g_gc) => {
                        assert_eq!(2, g_gc.0.len());
                        match g_gc.0[0] {
                            geo_types::Geometry::Point(ref p) => {
                                assert_eq!(4.0, p.0.x);
                                assert_eq!(6.0, p.0.y);
                            }
                            _ =>  assert!(false, "First element should be a point")
                        }
                        match g_gc.0[1] {
                            geo_types::Geometry::LineString(ref ls) => {
                                assert_eq!(2, ls.0.len());
                                assert_eq!(4.0, ls.0[0].0.x);
                                assert_eq!(6.0, ls.0[0].0.y);

                                assert_eq!(7.0, ls.0[1].0.x);
                                assert_eq!(10.0, ls.0[1].0.y);
                            }
                            _ =>  assert!(false, "First element should be a point")
                        }
                    }
                    _ => assert!(false, "Should be a Geom Collection")
                }
            }
            _ => assert!(false, "Should be a Geom Collection"),
        }
    }
}
