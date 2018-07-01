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
use tokenizer::{PeekableTokens, Token};
use FromTokens;
use types::CoordType;

#[derive(Default)]
pub struct Coord<T = f64>
where T: CoordType,
{
    pub x: T,
    pub y: T,
    pub z: Option<T>,
    pub m: Option<T>,
}

impl<T: CoordType> Coord<T> {
    pub fn new(x: T, y: T) -> Coord<T> {
        Coord{x, y, z: None, m: None}
    }
}

impl<T: CoordType> FromTokens<T> for Coord<T> {
    fn from_tokens(tokens: &mut PeekableTokens<T>) -> Result<Self, &'static str> {
        let x = match tokens.next() {
            Some(Token::Number(n)) => n,
            _ => return Err("Expected a number for the X coordinate"),
        };
        let y = match tokens.next() {
            Some(Token::Number(n)) => n,
            _ => return Err("Expected a number for the Y coordinate"),
        };
        Ok(Coord {
            x: x,
            y: y,
            z: None,
            m: None,
        })
    }
}


impl<T: CoordType> fmt::Display for Coord<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&format!("Coord({}, {})", self.x, self.y))
    }
}
