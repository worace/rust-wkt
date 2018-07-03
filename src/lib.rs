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
extern crate num_traits;

use std::default::Default;

use tokenizer::{PeekableTokens, Token, Tokens};
use types::GeometryCollection;
use types::LineString;
use types::MultiLineString;
use types::MultiPoint;
use types::MultiPolygon;
use types::Point;
use types::Polygon;
use types::CoordType;

mod tokenizer;

#[cfg(feature = "geo-types")]
mod towkt;
mod togeo;

pub mod types;

#[cfg(feature = "geo-types")]
pub use towkt::ToWkt;

pub enum Geometry<T: CoordType>
{
    Point(Point<T>),
    LineString(LineString<T>),
    Polygon(Polygon<T>),
    MultiPoint(MultiPoint<T>),
    MultiLineString(MultiLineString<T>),
    MultiPolygon(MultiPolygon<T>),
    GeometryCollection(GeometryCollection<T>),
}

impl<T: CoordType> Geometry<T> {
    fn from_word_and_tokens(word: &str, tokens: &mut PeekableTokens<T>) -> Result<Self, &'static str> {
        match word {
            "POINT" => {
                let x = <Point<T> as FromTokens<T>>::from_tokens_with_parens(tokens);
                x.map(|y| y.as_item())
            }
            "LINESTRING" => {
                let x = <LineString<T> as FromTokens<T>>::from_tokens_with_parens(tokens);
                x.map(|y| y.as_item())
            }
            "POLYGON" => {
                let x = <Polygon<T> as FromTokens<T>>::from_tokens_with_parens(tokens);
                x.map(|y| y.as_item())
            }
            "MULTIPOINT" => {
                let x = <MultiPoint<T> as FromTokens<T>>::from_tokens_with_parens(tokens);
                x.map(|y| y.as_item())
            }
            "MULTILINESTRING" => {
                let x = <MultiLineString<T> as FromTokens<T>>::from_tokens_with_parens(tokens);
                x.map(|y| y.as_item())
            }
            "MULTIPOLYGON" => {
                let x = <MultiPolygon<T> as FromTokens<T>>::from_tokens_with_parens(tokens);
                x.map(|y| y.as_item())
            }
            "GEOMETRYCOLLECTION" => {
                let x = <GeometryCollection<T> as FromTokens<T>>::from_tokens_with_parens(tokens);
                x.map(|y| y.as_item())
            }
            _ => Err("Invalid type encountered"),
        }
    }
}
pub struct Wkt<T>
where
    T: CoordType,
{
    pub items: Vec<Geometry<T>>,
}

impl<T: CoordType> Wkt<T> {
    pub fn new() -> Self {
        Wkt { items: vec![] }
    }

    pub fn add_item(&mut self, item: Geometry<T>) {
        self.items.push(item);
    }

    pub fn from_str(wkt_str: &str) -> Result<Self, &'static str> {
        let tokens = Tokens::from_str(wkt_str);
        Wkt::from_tokens(tokens)
    }

    fn from_tokens(tokens: Tokens<T>) -> Result<Self, &'static str> {
        let mut wkt = Wkt::new();
        let mut tokens = tokens.peekable();
        let word = match tokens.next() {
            Some(Token::Word(word)) => {
                if !word.is_ascii() {
                    return Err("Encountered non-ascii word");
                }
                word.to_ascii_uppercase()
            }
            None => return Ok(wkt),
            _ => return Err("Invalid WKT format"),
        };
        match Geometry::from_word_and_tokens(&word, &mut tokens) {
            Ok(item) => wkt.add_item(item),
            Err(s) => return Err(s),
        }
        Ok(wkt)
    }
}

trait FromTokens<T: CoordType>: Sized + Default {
    fn from_tokens(tokens: &mut PeekableTokens<T>) -> Result<Self, &'static str>;

    fn from_tokens_with_parens(tokens: &mut PeekableTokens<T>) -> Result<Self, &'static str> {
        match tokens.next() {
            Some(Token::ParenOpen) => (),
            Some(Token::Word(ref s)) if s.to_ascii_uppercase() == "EMPTY" => {
                return Ok(Default::default())
            }
            _ => return Err("Missing open parenthesis for type"),
        };
        let result = FromTokens::from_tokens(tokens);
        match tokens.next() {
            Some(Token::ParenClose) => (),
            _ => return Err("Missing closing parenthesis for type"),
        };
        result
    }

    fn comma_many<F>(f: F, tokens: &mut PeekableTokens<T>) -> Result<Vec<Self>, &'static str>
    where
        F: Fn(&mut PeekableTokens<T>) -> Result<Self, &'static str>,
    {
        let mut items = Vec::new();

        let item = try!(f(tokens));
        items.push(item);

        while let Some(&Token::Comma) = tokens.peek() {
            tokens.next(); // throw away comma

            let item = try!(f(tokens));
            items.push(item);
        }

        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use types::{MultiPolygon, Point};
    use {Geometry, Wkt};

    #[test]
    fn empty_string() {
        let wkt: Wkt<f64> = Wkt::from_str("").ok().unwrap();
        assert_eq!(0, wkt.items.len());
    }

    #[test]
    fn empty_items() {
        let mut wkt: Wkt<f64> = Wkt::from_str("POINT EMPTY").ok().unwrap();
        assert_eq!(1, wkt.items.len());
        match wkt.items.pop().unwrap() {
            Geometry::Point(Point(None)) => (),
            _ => unreachable!(),
        };

        let mut wkt: Wkt<f64> = Wkt::from_str("MULTIPOLYGON EMPTY").ok().unwrap();
        assert_eq!(1, wkt.items.len());
        match wkt.items.pop().unwrap() {
            Geometry::MultiPolygon(MultiPolygon(polygons)) => assert_eq!(polygons.len(), 0),
            _ => unreachable!(),
        };
    }

    #[test]
    fn test_to_wkt() {
        assert!(true);
    }
}
