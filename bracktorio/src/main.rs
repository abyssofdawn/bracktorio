use bracket_lib::terminal::{main_loop, BResult};
use bracket_lib::prelude::*;

fn main() -> BResult<()> {
    let context = match bracktorio::create_context() {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };

    if let Err(e) = main_loop(context , bracktorio::game::State::new()){
        panic!("{}", e);
    } else {
        Ok(())
    }
}



