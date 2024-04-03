use super::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Tile {
    pub glyph: bracket_lib::prelude::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}