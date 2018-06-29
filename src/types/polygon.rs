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

use tokenizer::PeekableTokens;
use types::linestring::LineString;
use FromTokens;
use Geometry;
use num_traits::Float;

#[derive(Default)]
pub struct Polygon<T: Float>(pub Vec<LineString<T>>);

impl<T: Float> Polygon<T> {
    pub fn as_item(self) -> Geometry<T> {
        Geometry::Polygon(self)
    }
}

impl<T: Float> FromTokens for Polygon<T> {
    fn from_tokens(tokens: &mut PeekableTokens) -> Result<Self, &'static str> {
        let result =
            FromTokens::comma_many(<LineString as FromTokens>::from_tokens_with_parens, tokens);
        result.map(|vec| Polygon(vec))
    }
}

#[cfg(test)]
mod tests {
    use super::Polygon;
    use {Geometry, Wkt};

    #[test]
    fn basic_polygon() {
        let mut wkt = Wkt::from_str("POLYGON ((8 4, 4 0, 0 4, 8 4), (7 3, 4 1, 1 4, 7 3))")
            .ok()
            .unwrap();
        assert_eq!(1, wkt.items.len());
        let lines = match wkt.items.pop().unwrap() {
            Geometry::Polygon(Polygon(lines)) => lines,
            _ => unreachable!(),
        };
        assert_eq!(2, lines.len());
    }
}
