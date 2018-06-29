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
use types::point::Point;
use FromTokens;
use Geometry;
use num_traits::Float;

#[derive(Default)]
pub struct MultiPoint<T: Float>(pub Vec<Point<T>>);

impl<T: Float> MultiPoint<T> {
    pub fn as_item(self) -> Geometry<T> {
        Geometry::MultiPoint(self)
    }
}

impl<T: Float> FromTokens for MultiPoint<T> {
    fn from_tokens(tokens: &mut PeekableTokens) -> Result<Self, &'static str> {
        let result = FromTokens::comma_many(<Point as FromTokens>::from_tokens_with_parens, tokens);
        result.map(|vec| MultiPoint(vec))
    }
}

#[cfg(test)]
mod tests {
    use super::MultiPoint;
    use {Geometry, Wkt};

    #[test]
    fn basic_multipoint() {
        let mut wkt = Wkt::from_str("MULTIPOINT ((8 4), (4 0))").ok().unwrap();
        assert_eq!(1, wkt.items.len());
        let points = match wkt.items.pop().unwrap() {
            Geometry::MultiPoint(MultiPoint(points)) => points,
            _ => unreachable!(),
        };
        assert_eq!(2, points.len());
    }
}
