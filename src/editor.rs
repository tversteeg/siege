use siege::*;

use line_drawing::*;

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
    pos: (i32, i32),
    size: (i32, i32),
    mouse: (i32, i32, bool),
    status: Status,

    engine: Engine,
}

impl Editor {
    pub fn new(x: i32, y: i32) -> Self {
        Editor {
            pos: (x, y),
            size: (400, 400),
            mouse: (0, 0, false),
            status: Status::None,

            engine: Engine::new(x as f64 + 100.0, y as f64 + 100.0),
        }
    }

    pub fn update(&mut self, mouse: (i32, i32, bool), action: Action) {
        if self.mouse.2 != mouse.2 {
            let status = self.status;
            self.status = match status {
                Status::Draw(x, y) => {
                    if let Action::DrawBeam(material) = action {
                        self.engine.add_beam((x, y), (mouse.0, mouse.1), material);
                    }

                    Status::None
                },
                Status::None => Status::Draw(mouse.0, mouse.1)
            };
        }

        self.mouse = mouse;
    }

    pub fn draw(&mut self, dst: &mut [u32], dst_width: usize) {
        for x in self.pos.0 .. self.pos.0 + self.size.0 {
            for y in self.pos.1 .. self.pos.1 + self.size.1 {
                dst[x as usize + y as usize * dst_width] = 0xFFFFFF;
            }
        }

        match self.status {
            Status::Draw(start_x, start_y) => {
                for (x, y) in Bresenham::new((start_x, start_y), (self.mouse.0, self.mouse.1)) {
                    if x < self.pos.0 || x >= self.pos.0 + self.size.0 || y < self.pos.1 || y >= self.pos.1 + self.size.1 {
                        continue;
                    }
                    dst[x as usize + y as usize * dst_width] = 0x00FF00;
                }
            },
            _ => ()
        }

        self.engine.render_to_buffer(dst, dst_width);
    }
}
