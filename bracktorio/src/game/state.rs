use bracket_lib::terminal::*;
use specs::prelude::*;

use crate::objects::*;

pub struct State {
    pub ecs: World
}

impl State { 
    fn run_systems(&mut self) {
        let mut vis = DisplayObjects {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }

    pub fn new() -> Self {
        let mut ecs = World::new();

        ecs.register::<Object>();

        ecs.create_entity()
            .with(Object {
                id: 0,
            })
            .build();

        Self { 
            ecs: ecs
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.run_systems();
        let objects = self.ecs.read_storage::<Object>();
        for object in (&objects).join() {
            ctx.print(0, 0, object.id.to_string().as_str());
        }
    }
}

