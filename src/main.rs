mod decoder;
mod extractor;
mod path_resolver;
mod platform;
mod app;

use app::App;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new();
    app.run(&args);
}
