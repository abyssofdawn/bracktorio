pub use specs::*;
pub use specs_derive::*;
pub use bracket_lib::prelude::*;


pub mod miner;
pub use miner::*;
pub mod crafter;
pub use crafter::*;

#[derive(Component)]
pub struct Object {
    pub id : usize,
}


#[derive(Component)]
pub struct RenderObject {
    pub sprite: MultiTileSprite,
    pub z : i32,
}

#[derive(Component)]
pub struct Machine {
    pub in_slots: Vec<Slot>,
    pub out_slots: Vec<Slot>,
    pub recipe: Option<Recipe>,
    pub progress: usize,
}

#[derive(Clone)]
pub struct Slot {
    pub item: Option<Item>,
    pub amount: i32,
}

pub struct Recipe {
    pub input: Vec<Slot>,
    pub output: Vec<Slot>,
    pub craft_ticks: usize
}

#[derive(Component, Clone)]
pub struct Item {
    pub name : String,
    pub icon: Tile
}