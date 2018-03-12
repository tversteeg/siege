use siege::*;

use line_drawing::*;

const SCALE: i32 = 8;
const EDITOR_SIZE: i32 = 40 * SCALE;

pub enum Action {
    DrawBeam(Material),
    DrawWheel(Material)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Status {
    Draw(i32, i32),
    None
}

pub struct Editor {
    rect: (i32, i32, i32, i32),
    mouse: (i32, i32, bool),
    status: Status,

    engine: Engine,
}

impl Editor {
    pub fn new(x: i32, y: i32) -> Self {
        Editor {
            rect: (x, y, x + EDITOR_SIZE, y + EDITOR_SIZE),
            mouse: (0, 0, false),
            status: Status::None,

            engine: Engine::new((x + EDITOR_SIZE) as f64, y as f64),
        }
    }

    pub fn update(&mut self, mouse: (i32, i32, bool), action: Action) {
        let x_in_rect = mouse.0 >= self.rect.0 && mouse.0 < self.rect.2;
        let y_in_rect = mouse.1 >= self.rect.1 && mouse.1 < self.rect.3;
        if x_in_rect && y_in_rect && self.mouse.2 != mouse.2 {
            let status = self.status;
            self.status = match status {
                Status::Draw(x, y) => {
                    if let Action::DrawBeam(material) = action {
                        let scaled_draw = ((x - self.rect.0) / SCALE, (y - self.rect.1) / SCALE);
                        let scaled_mouse = ((mouse.0 - self.rect.0) / SCALE, (mouse.1 - self.rect.1) / SCALE);
                        self.engine.add_beam(scaled_draw, scaled_mouse, material);
                    }

                    Status::None
                },
                Status::None => Status::Draw(mouse.0, mouse.1)
            };
        }

        self.mouse = mouse;
    }

    pub fn draw(&mut self, dst: &mut [u32], dst_width: usize) {
        let engine_preview_x = (self.rect.2 as usize, (self.rect.2 + (EDITOR_SIZE / SCALE)) as usize);
        let engine_preview_y = (self.rect.1 as usize, (self.rect.1 + (EDITOR_SIZE / SCALE)) as usize);

        // Render the white background small
        for x in engine_preview_x.0 .. engine_preview_x.1 {
            for y in engine_preview_y.0 .. engine_preview_y.1 {
                dst[x + y * dst_width] = 0xFFFFFF;
            }
        }

        // Render the engine
        self.engine.render_to_buffer(dst, dst_width);

        // Upscale the preview
        let mut new = (self.rect.0 as usize, self.rect.1 as usize);
        for x in engine_preview_x.0 .. engine_preview_x.1 {
            for y in engine_preview_y.0 .. engine_preview_y.1 {
                let color = dst[x + y * dst_width];

                for x2 in 0..SCALE as usize {
                    for y2 in 0..SCALE as usize {
                        dst[new.0 + x2 + (new.1 + y2) * dst_width] = color;
                    }
                }
                new.1 += SCALE as usize;
            }
            new.0 += SCALE as usize;
            new.1 = self.rect.1 as usize;
        }

        match self.status {
            Status::Draw(start_x, start_y) => {
                for (x, y) in Bresenham::new((start_x, start_y), (self.mouse.0, self.mouse.1)) {
                    if x < self.rect.0 || x >= self.rect.2 || y < self.rect.1 || y >= self.rect.3 {
                        continue;
                    }
                    dst[x as usize + y as usize * dst_width] = 0x00FF00;
                }
            },
            _ => ()
        }
    }
}
