pub use specs::*;
pub use specs_derive::*;
pub use bracket_lib::prelude::*;

pub mod display_object;
pub use display_object::*;

#[derive(Component)]
pub struct Object {
    pub id : usize,
}