use bracket_lib::terminal::*;

pub mod objects;
pub mod world;
pub mod game;
pub mod util;


pub fn create_context() -> BResult<BTerm> {
    embedded_resource!(FONT_10X10, "../resources/10x10.png");

    link_resource!(FONT_10X10, "10x10.png");
    let font_10x10 = "10x10.png";
    BTermBuilder::new()
    .with_tile_dimensions(10, 10)
    .with_dimensions(80, 50)
    .with_font(font_10x10, 10, 10)
    .with_simple_console(80, 50, font_10x10)
    .with_gutter(10)
    .with_title("Bractorio")
    .with_automatic_console_resize(true)
    .build()
}

