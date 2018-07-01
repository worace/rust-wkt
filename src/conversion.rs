extern crate geo_types;

use types::Coord;
use types::CoordType;
use std::convert::From;
// use types::GeometryCollection;
use types::LineString;
// use types::MultiLineString;
// use types::MultiPoint;
// use types::MultiPolygon;
use types::Point;
// use types::Polygon;
use Geometry;
// use Wkt;

// trait CoordCommon = geo_types::CoordinateType + CoordType;

// Tried to implement std::convert::From for this, but ran into issues with
// https://github.com/rust-lang/rust/issues/24745 so just doing it as a new trait ATM
pub trait ToGeo<T: geo_types::CoordinateType> {
    fn to_geo(&self) -> Result<geo_types::Geometry<T>, &'static str>;
}

fn coord_to_g_coord<T: geo_types::CoordinateType + CoordType>(coord: &Coord<T>) -> geo_types::Coordinate<T> {
    geo_types::Coordinate { x: coord.x, y: coord.y}
}

fn linestring_to_g_linestring<T: geo_types::CoordinateType + CoordType>(ls: &LineString<T>) -> geo_types::LineString<T> {
    let mut g_coords: Vec<geo_types::Coordinate<T>> = vec![];
    for c in &ls.0 {
        g_coords.push(coord_to_g_coord(c));
    }
    g_coords.into_iter().collect()
}

impl<T: geo_types::CoordinateType + CoordType> ToGeo<T> for Geometry<T>
{
    fn to_geo(self: &Geometry<T>) -> Result<geo_types::Geometry<T>, &'static str> {
        match *self {
            Geometry::Point(Point(Some(ref c))) => {
                let g_coord = coord_to_g_coord(c);
                let g_point = geo_types::Point(g_coord);
                Ok(geo_types::Geometry::Point(g_point))
            }
            Geometry::LineString(ref ls) => Ok(geo_types::Geometry::LineString(linestring_to_g_linestring(ls))),
            // Geometry::Polygon(Polygon(ref rings)) => {
            //     let mut g_rings: Vec<geo_types::LineString<T>> = vec![];
            //     for r in rings {
            //         g_rings.push(linestring_to_g_linestring(r));
            //     }
            //     Ok(geo_types::Polygon(g_rings))
            // }
            _ => Err("not implemented")
        }
        // let x: T = 0.0;
        // let y: T = 0.0;
        // let p: geo_types::Point<T> = geo_types::Point::new(x, y);
        // geo_types::Geometry::Point(p)
    }
}

#[cfg(test)]
mod tests {
    extern crate geo_types;
    use types::Coord;
    use types::Point;
    use types::LineString;
    use Geometry;
    use conversion::ToGeo;
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
    #[ignore]
    fn converting_polygon_to_geo() {
        let wkt: Wkt<f64> = Wkt::from_str("POLYGON ((8 4, 4 0, 0 4, 8 4), (7 3, 4 1, 1 4, 7 3))").ok().unwrap();
        assert_eq!(1, wkt.items.len());
        let poly = &wkt.items[0];
        match poly {
            Geometry::Polygon(_) => {
                let g_poly = poly.to_geo().unwrap().as_polygon().unwrap();

                // let mut coords = g_ls.into_iter();

                // let c1 = coords.next().unwrap();
                // assert_eq!(10.0, c1.x());
                // assert_eq!(-20.0, c1.y());

                // let c2 = coords.next().unwrap();
                // assert_eq!(0.0, c2.x());
                // assert_eq!(-0.5, c2.y());
            }
            _ => assert!(false, "should be linestring!"),
        }
    }
}
