use bracket_lib::terminal::{main_loop, BResult};

fn main() -> BResult<()> {
    let context = match bracktorio::create_context() {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };

    return main_loop(context , bracktorio::game::State::new());
}



