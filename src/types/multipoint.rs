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

use tokenizer::{PeekableTokens, Token};
use types::FromTokens;
use types::point::Point;
use WktItem;


pub struct MultiPoint {
    pub points: Vec<Point>
}

impl MultiPoint {
    pub fn as_item(self) -> WktItem {
        WktItem::MultiPoint(self)
    }
}

impl FromTokens for MultiPoint {
    fn from_tokens(tokens: &mut PeekableTokens) -> Result<Self, &'static str> {
        let mut points = Vec::new();

        let x: Result<Point, _> = FromTokens::from_tokens_with_parens(tokens);

        points.push(match x {
            Ok(p) => p,
            Err(s) => return Err(s),
        });

        while let Some(&Token::Comma) = tokens.peek() {
            tokens.next();  // throw away comma

            let x: Result<Point, _> = FromTokens::from_tokens_with_parens(tokens);

            points.push(match x {
                Ok(p) => p,
                Err(s) => return Err(s),
            });
        }

        Ok(MultiPoint {points: points})
    }
}