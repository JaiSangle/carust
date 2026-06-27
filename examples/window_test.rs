use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut window = Window::new(
        "Test",
        640,
        400,
        WindowOptions {
        resize: true,
        ..WindowOptions::default()
    })
    .expect("Unable to open Window");

    while window.is_open() {
        if window.is_key_down(Key::Escape){
            break;
        }

        window.update();
    }
}