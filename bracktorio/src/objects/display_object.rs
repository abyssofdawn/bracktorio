use super::*;

pub struct DisplayObjects {}

impl<'a> System<'a> for DisplayObjects {
    type SystemData = (ReadStorage<'a, Object>,);

    fn run(&mut self, data: Self::SystemData) {
        let (objects,) = data;
        for (object, ) in (&objects,).join() {
            console::log(format!("{}", object.id));
        }
    }
}