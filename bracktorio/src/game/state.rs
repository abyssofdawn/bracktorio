use bracket_lib::terminal::*;
use specs::prelude::*;

use crate::objects::*;

use super::*;

pub struct State {
    pub ecs: World,
    pub world_pos: Position,
    pub time: f32,
    pub ticks: usize,
    pub items: Vec<Item>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Step,
    Wait
}

impl State { 
    fn run_systems(&mut self) {
        let mut cra = CrafterTicker{};
        cra.run_now(&self.ecs);
        self.ecs.maintain();
    }

    pub fn new() -> Self {
        let item1 = Item{
            name: "Stone".to_string(),
            icon: Tile {
                glyph: 65,
                fg: RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
                bg: RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
            },
        };
        let item2 = Item{
            name: "Glass".to_string(),
            icon: Tile {
                glyph: 65,
                fg: RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
                bg: RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
            },
        };
        let recipe = Recipe {
            craft_ticks: 100,
            input: vec![Slot{item: Some(item1.clone()), amount: 1}],
            output: vec![Slot{item: Some(item2.clone()), amount: 1}],
        };




        let mut ecs = World::new();

        ecs.register::<Object>();
        ecs.register::<RenderObject>();
        ecs.register::<Position>();
        ecs.register::<Miner>();
        ecs.register::<Crafter>();
        ecs.register::<Item>();
        ecs.register::<Machine>();

        ecs.create_entity()
            .with(Object {
                id: 0,
            })
            .with(Position {
                x: 0,
                y: 0,
            })
            .with(RenderObject {
                sprite: MultiTileSprite::from_string_colored(
                    " ▲ ◄☼► ▼ ",
                    3,
                    3,
                    &[
                        RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
                        RGBA::from_f32(0.0, 1.0, 0.0, 1.0),
                        RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
                        RGBA::from_f32(0.0, 1.0, 0.0, 1.0),
                        RGBA::from_f32(1.0, 1.0, 0.0, 1.0),
                        RGBA::from_f32(0.0, 1.0, 0.0, 1.0),
                        RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
                        RGBA::from_f32(0.0, 1.0, 0.0, 1.0),
                        RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
                    ],
                    &[RGBA::from_f32(0.0, 0.0, 0.0, 1.0); 9],
                ),
                z: 0,
            })
            .with(Crafter {})
            //.with(Machine {
            //    in_slots: vec![Slot {
            //        item: Some(item1),
            //        amount: 100,
            //    }; 1],
            //    out_slots: vec![Slot {
            //        item: Some(item2),
            //        amount: 0,
            //    }; 1],
            //    progress: 0,
            //    recipe: Some(recipe)
            //})
            .build();

        ecs.insert(RunState::Step);

        Self { 
            ecs: ecs,
            world_pos: Position {
                x: 0,
                y: 0,
            },
            time: 0.0,
            ticks: 0,
            items: vec![item1, item2],
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        self.time += ctx.frame_time_ms;
        if newrunstate == RunState::Step {
            newrunstate = RunState::Wait;
            self.run_systems();
        }
        let obj  =(
            &self.ecs.read_storage::<Object>(), 
            &self.ecs.read_storage::<RenderObject>(),
            &mut self.ecs.write_storage::<Position>());

        match ctx.key {
            None => {},
            Some(key) => match key {
                VirtualKeyCode::A | VirtualKeyCode::Left => self.world_pos.x += 1,
                VirtualKeyCode::S | VirtualKeyCode::Down => self.world_pos.y -= 1,
                VirtualKeyCode::W | VirtualKeyCode::Up => self.world_pos.y += 1,
                VirtualKeyCode::D | VirtualKeyCode::Right => self.world_pos.x -= 1,
                _ => {}
            }
        }


        let mut objects: Vec<(&Object, &RenderObject, &Position)> = Vec::new();

        let mut batch = DrawBatch::new();
        batch.target(0);


        for (object, render_object, position) in (obj.0, obj.1, obj.2).join() {
            objects.push((object, render_object, position));
        }

        objects.sort_by(|a, b| a.1.z.cmp(&b.1.z));

        for (_, robj, pos) in objects.iter() {
            robj.sprite.add_to_batch(&mut batch, Point {
                x: pos.x + self.world_pos.x - 1,
                y: pos.y + self.world_pos.y,
            })
        }

        batch.submit(0).expect("batch error");

        render_draw_buffer(ctx, ).expect("render error");

        for (machine, _render_object, position) in 
            (   &self.ecs.read_storage::<Machine>(), 
                &self.ecs.read_storage::<RenderObject>(), 
                &mut self.ecs.write_storage::<Position>()
            )
        .join() { }

        ctx.set(40, 25, RGB::named(CYAN2), RGB::named(BLACK), 2);
        ctx.print(0, 49, self.time);
        ctx.print(0, 48, self.ticks);

        if self.time / self.ticks as f32 >= 1000.0 / 60.0 {
            self.ticks+=1;
            newrunstate = RunState::Step;
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
    }
}

