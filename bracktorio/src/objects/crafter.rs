use super::*;

#[derive(Component)]
pub struct Crafter{}

pub struct CrafterTicker {}

impl <'a> System<'a> for CrafterTicker {
    type SystemData = (
        ReadStorage<'a, Crafter>,
        WriteStorage<'a, Machine>,
        Entities<'a>,

    );

    fn run(&mut self, data: Self::SystemData) {
        let (crafters, mut machines, _entities) = data;

        //loop through crafters
        for (_crafter, machine) in (&crafters, &mut machines).join() {
            //if the crafter has a recipe
            if let Some(recipe) = &machine.recipe {
                //flag if the recipe can be executed
                let mut work = true;
                let done = machine.progress >= recipe.craft_ticks;
                //if the machine input and output slot count is less than the recipe slot count, it doesn't work
                if machine.in_slots.len() >= recipe.input.len() && machine.out_slots.len() >= recipe.output.len() {
                    //loop through recipe input slots
                    for slot in recipe.input.iter() {
                        //get the slot item
                        if let Some(s) = &slot.item {
                            let mut name_found = false;
                            //loop through machine input slots
                            for inslot in machine.in_slots.iter() {
                                //get the slot item
                                if let Some(m) = &inslot.item {
                                    if m.name == s.name {
                                        name_found = true;
                                    }
                                    //if the slot can safely subtract the amount
                                    if slot.amount > inslot.amount {
                                        work = false;
                                    }
                                }
                            }
                            //if the slot name wasn't found
                            if !name_found {
                                work = false;
                            }
                        }
                    }
                } else {
                    work = false;
                }
                //since it runs once a tick, increase the progress by total number of ticks needed 
                if work {
                    machine.progress += 1;
                }
                // assuming the recipe can be executed
                if done && work {
                    machine.progress = 0;
                    //loop through recipe input slots
                    for slot in recipe.input.iter() {
                        if let Some(s) = &slot.item {
                            //loop through machine input slots
                            for inslot in machine.in_slots.iter_mut() {
                                if let Some(m) = &inslot.item {
                                    //subtract the amount needed from the slot
                                    if m.name == s.name {
                                        inslot.amount -= slot.amount;
                                    }
                                }
                            }
                        }
                    }
                    //loop through recipe output slots
                    for slot in recipe.output.iter() {
                        if let Some(s) = &slot.item {
                            //loop through machine output slots
                            let mut name_found = false;
                            for outslot in machine.out_slots.iter_mut() {
                                //if the slot item name is same, then set flag
                                if let Some(m) = &outslot.item {
                                    if m.name == s.name {
                                        name_found = true;
                                    }
                                }
                                //name is found, then increase output slot count and stop looking
                                if name_found {
                                    outslot.amount += slot.amount;
                                    break;
                                }          
                            }
                            //if the slot name wasn't found, then add it (and amount by default is now slot.amount)
                            if !name_found {
                                for outslot in machine.out_slots.iter_mut() {
                                    if outslot.item.is_none() {
                                        outslot.item = Some(s.clone());
                                        outslot.amount = slot.amount;
                                        break;
                                    }
                                }
                            }
                        }                        
                    }
                }
            }
        }
    }

}