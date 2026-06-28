use crate::app::App;
mod app;
mod camera;
mod frame;
mod network;
mod renderer;

fn main() {
    let mut app = App::new();
    app.run();
}
