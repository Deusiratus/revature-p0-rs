use revature_p0_rs::App;

fn main() {
    use std::env;

    env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    app.run();
}
