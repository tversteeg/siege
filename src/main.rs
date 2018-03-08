extern crate siege;
extern crate minifb;
extern crate direct_gui;

use minifb::*;
use direct_gui::*;
use direct_gui::controls::*;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

fn main() {
    let mut buffer: Vec<u32> = vec![0x00FFFFFF; WIDTH * HEIGHT];

    let options = WindowOptions {
        scale: Scale::X2,
        ..WindowOptions::default()
    };
    let mut window = Window::new("Siege Editor - ESC to exit", WIDTH, HEIGHT, options).expect("Unable to open window");

    let mut gui = Gui::new((WIDTH as i32, HEIGHT as i32));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut cs = ControlState {
            ..ControlState::default()
        };

        window.get_mouse_pos(MouseMode::Pass).map(|mouse| {
            cs.mouse_pos = (mouse.0 as i32, mouse.1 as i32);
            cs.mouse_down = window.get_mouse_down(MouseButton::Left);

            gui.update(&cs);
        });

        gui.draw_to_buffer(&mut buffer);

        window.update_with_buffer(&buffer).unwrap();
    }
}
