use crate::{Engine, Tile};
use lyon::{math::point, path::Path};

/// Implemented to create vector paths using lyon for the engines.
pub trait ToVector {
    /// Exports an vector based representation of a siege engine.
    ///
    /// The scale is a multiplier for the output size.
    fn to_vector(&self, scale: f32) -> Path;
}

impl ToVector for Engine {
    fn to_vector(&self, scale: f32) -> Path {
        let mut builder = Path::builder();

        // Create a path for all tiles
        self.to_grid().enumerate().for_each(|(coord, tile)| {
            let x = coord.x as f32 * scale;
            let y = coord.y as f32 * scale;
            match tile {
                Tile::Wall => {
                    // Fill the whole square
                    builder.move_to(point(x, y));
                    builder.line_to(point(x + scale, y));
                    builder.line_to(point(x + scale, y + scale));
                    builder.line_to(point(x, y + scale));
                    builder.close();
                }
                Tile::Wheel => {
                    // Create a circle
                }
                _ => (),
            }
        });

        builder.build()
    }
}
