// Copyright 2014-2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use tokenizer::PeekableTokens;
use types::CoordType;
use types::coord::Coord;
use FromTokens;
use Geometry;

#[derive(Default)]
pub struct Point<T: CoordType>(pub Option<Coord<T>>);

impl<T: CoordType> Point<T> {
    pub fn as_item(self) -> Geometry<T> {
        Geometry::Point(self)
    }
}

impl<T: CoordType> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("POINT")
    }
}

impl<T: CoordType> FromTokens<T> for Point<T> {
    fn from_tokens(tokens: &mut PeekableTokens<T>) -> Result<Self, &'static str> {
        let result = <Coord<T> as FromTokens<T>>::from_tokens(tokens);
        result.map(|coord| Point(Some(coord)))
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use {Geometry, Wkt};

    #[test]
    fn basic_point() {
        let mut wkt: Wkt<f64> = Wkt::from_str("POINT (10 -20)").ok().unwrap();
        assert_eq!(1, wkt.items.len());
        let coord = match wkt.items.pop().unwrap() {
            Geometry::Point(Point(Some(coord))) => coord,
            _ => unreachable!(),
        };
        assert_eq!(10.0, coord.x);
        assert_eq!(-20.0, coord.y);
        assert_eq!(None, coord.z);
        assert_eq!(None, coord.m);
    }

    #[test]
    fn basic_point_whitespace() {
        let mut wkt: Wkt<f64> = Wkt::from_str(" \n\t\rPOINT \n\t\r( \n\r\t10 \n\t\r-20 \n\t\r) \n\t\r")
            .ok()
            .unwrap();
        assert_eq!(1, wkt.items.len());
        let coord = match wkt.items.pop().unwrap() {
            Geometry::Point(Point(Some(coord))) => coord,
            _ => unreachable!(),
        };
        assert_eq!(10.0, coord.x);
        assert_eq!(-20.0, coord.y);
        assert_eq!(None, coord.z);
        assert_eq!(None, coord.m);
    }

    #[test]
    fn invalid_points() {
        let wkts = vec!["POINT ()", "POINT (10)", "POINT 10", "POINT (10 -20 40)"];
        for s in wkts {
            let parsed: Result<Wkt<f64>, _> = Wkt::from_str(s);
            parsed.err().unwrap();
        }
    }
}
