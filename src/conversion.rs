extern crate geo_types;

use types::Coord;
use types::CoordType;
use std::convert::From;
// use types::GeometryCollection;
// use types::LineString;
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
    fn to_geo(self) -> Result<geo_types::Geometry<T>, &'static str>;
}

fn coord_to_g_coord<T: geo_types::CoordinateType + CoordType>(coord: Coord<T>) -> geo_types::Coordinate<T> {
    geo_types::Coordinate { x: coord.x, y: coord.y}
}

impl<T: geo_types::CoordinateType + CoordType> ToGeo<T> for Geometry<T>
{
    fn to_geo(self: Geometry<T>) -> Result<geo_types::Geometry<T>, &'static str> {
        match self {
            Geometry::Point(Point(Some(c))) => {
                let g_coord = coord_to_g_coord(c);
                let g_point = geo_types::Point(g_coord);
                Ok(geo_types::Geometry::Point(g_point))
            }
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
    use types::Point;
    use types::Coord;
    use Geometry;
    use conversion::ToGeo;
    // use conversion::From;
    // use self::From;
    // use self::geo_types;

    #[test]
    fn converting_geo() {
        let c: Coord<f64> = Coord::new(0.0, 0.0);
        println!("***********");
        println!("{:?}", c.x);
        let p = Geometry::Point(Point(Some(c)));
        // let poly: geo_types::Geometry::Polygon(geo_types::Polygon<f64>) = geo_types::Polygon::new(coords.into(), vec![]);

        let geo_geom: geo_types::Geometry<f64> = p.to_geo().unwrap();
        println!("{:?}", geo_geom);
        // assert_eq!(1,2);
    }
}
