// Copyright 2015 The GeoRust Developers
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
use types::polygon::Polygon;
use FromTokens;
use Geometry;
use num_traits::Float;

#[derive(Default)]
pub struct MultiPolygon<T: Float>(pub Vec<Polygon<T>>);

impl<T: Float> MultiPolygon<T> {
    pub fn as_item(self) -> Geometry<T> {
        Geometry::MultiPolygon(self)
    }
}

impl<T: Float> FromTokens for MultiPolygon<T> {
    fn from_tokens(tokens: &mut PeekableTokens) -> Result<Self, &'static str> {
        let result =
            FromTokens::comma_many(<Polygon as FromTokens>::from_tokens_with_parens, tokens);
        result.map(|vec| MultiPolygon(vec))
    }
}

#[cfg(test)]
mod tests {
    use super::MultiPolygon;
    use {Geometry, Wkt};

    #[test]
    fn basic_multipolygon() {
        let mut wkt = Wkt::from_str("MULTIPOLYGON (((8 4)), ((4 0)))")
            .ok()
            .unwrap();
        assert_eq!(1, wkt.items.len());
        let polygons = match wkt.items.pop().unwrap() {
            Geometry::MultiPolygon(MultiPolygon(polygons)) => polygons,
            _ => unreachable!(),
        };
        assert_eq!(2, polygons.len());
    }
}
