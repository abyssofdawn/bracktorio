use crate::game::*;

use super::*;

#[derive(Component)]
pub struct Miner{}

pub struct MinerTicker {}

impl <'a> System<'a> for MinerTicker {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Miner>,
        Entities<'a>,

    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, miners, _entities) = data;

        for (position, _miner) in (&mut positions, &miners).join() {
            position.x += 1;
        }
    }

}