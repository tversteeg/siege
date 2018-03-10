extern crate siege;
extern crate minifb;
extern crate direct_gui;

use minifb::*;
use direct_gui::*;
use direct_gui::controls::*;
use siege::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0x00222034; WIDTH * HEIGHT];

    let mut window = Window::new("Siege Editor - ESC to exit", WIDTH, HEIGHT, WindowOptions::default()).expect("Unable to open window");

    let mut gui = Gui::new((WIDTH as i32, HEIGHT as i32));

    let font = gui.default_font();

    let wood_button = gui.register(Button::new((32, 32), Color::from_u32(0x8F563B)).with_pos(4, 4));
    gui.register(Label::new(font).with_pos(40, 4).with_text("Wood"));

    let metal_button = gui.register(Button::new((32, 32), Color::from_u32(0x847E87)).with_pos(4, 40));
    gui.register(Label::new(font).with_pos(40, 40).with_text("Metal"));

    let mut selected_material = Material::Rope;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut cs = ControlState {
            ..ControlState::default()
        };

        window.get_mouse_pos(MouseMode::Pass).map(|mouse| {
            cs.mouse_pos = (mouse.0 as i32, mouse.1 as i32);
            cs.mouse_down = window.get_mouse_down(MouseButton::Left);

            {
                let wood_button: &Button<Flat> = gui.get(wood_button).unwrap();
                if !cs.mouse_down && wood_button.pressed() {
                    selected_material = Material::Wood;
                }

                let metal_button: &Button<Flat> = gui.get(metal_button).unwrap();
                if !cs.mouse_down && metal_button.pressed() {
                    selected_material = Material::Metal;
                }
            }

            gui.update(&cs);
        });

        gui.draw_to_buffer(&mut buffer);

        window.update_with_buffer(&buffer).unwrap();
    }
}
