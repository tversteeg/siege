mod render;

use crate::render::Render;
use anyhow::Result;
use lyon::path::{builder::Build, Path};
use miniquad::{
    conf::{Conf, Loading},
    Context, EventHandler, UserData,
};
use siege::Generator;

type Vec2 = vek::Vec2<f64>;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

/// Our app state.
struct App {
    /// Our wrapper around the OpenGL calls.
    render: Render,
}

impl App {
    /// Setup the ECS and load the systems.
    pub fn new(ctx: &mut Context) -> Result<Self> {
        // Setup the OpenGL render part
        let mut render = Render::new(ctx);

        // Build a Path for the rust logo.
        let mut builder = Path::builder();
        let logo_mesh = render.upload_path(builder.build().iter());

        logo_mesh.add_instance(Vec2::zero());

        Ok(Self { render })
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        // Render the buffer
        self.render.render(ctx);
    }
}

fn main() {
    let mut generator = Generator::default();
    let engine = generator
        .generate_skeleton(20, 20, 100, &mut rand::thread_rng())
        .unwrap();
    println!("{}", engine.to_ascii());

    /*
    miniquad::start(
        Conf {
            window_title: concat!("siege lyon example - ", env!("CARGO_PKG_VERSION")).to_string(),
            window_width: WIDTH as i32,
            window_height: HEIGHT as i32,
            loading: Loading::Embedded,
            sample_count: 4,
            ..Default::default()
        },
        |mut ctx| {
            UserData::owning(
                App::new(&mut ctx).expect("Setting up app state failed"),
                ctx,
            )
        },
    );
        */
}
