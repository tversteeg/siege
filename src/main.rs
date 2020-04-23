mod render;

use crate::render::Render;
use anyhow::Result;
use lyon::{
    extra::rust_logo::build_logo_path,
    path::{builder::Build, Path},
};
use miniquad::{
    conf::{Conf, Loading},
    Context, EventHandler, UserData,
};

type Vec2 = vek::Vec2<f64>;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

/// Our game state.
struct Game {
    /// Our wrapper around the OpenGL calls.
    render: Render,
}

impl Game {
    /// Setup the ECS and load the systems.
    pub fn new(ctx: &mut Context) -> Result<Self> {
        // Setup the OpenGL render part
        let mut render = Render::new(ctx);

        // Build a Path for the rust logo.
        let mut builder = Path::builder().with_svg();
        build_logo_path(&mut builder);
        let logo_mesh = render.upload(builder.build().iter());

        for x in -100..100 {
            for y in -100..100 {
                logo_mesh.add_instance(Vec2::new(x as f64 * 100.0, y as f64 * 100.0));
            }
        }

        Ok(Self { render })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        // Render the buffer
        self.render.render(ctx);
    }
}

fn main() {
    miniquad::start(
        Conf {
            window_title: concat!("siege - ", env!("CARGO_PKG_VERSION")).to_string(),
            window_width: WIDTH as i32,
            window_height: HEIGHT as i32,
            loading: Loading::Embedded,
            sample_count: 4,
            ..Default::default()
        },
        |mut ctx| {
            UserData::owning(
                Game::new(&mut ctx).expect("Setting up game state failed"),
                ctx,
            )
        },
    );
}
