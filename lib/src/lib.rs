extern crate line_drawing;

use line_drawing::{Point, Bresenham, BresenhamCircle};

/// The material that the parts are made of.
/// This determines how they get rendered and their strength.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Material {
    Wood,
    Metal
}

/// The parts of which a siege engine is build up.
#[derive(Debug)]
pub enum Part {
    Beam(Beam),
    Wheel(Wheel)
}

/// A rectangular rod which is a basically a line with a material.
#[derive(Debug)]
pub struct Beam {
    pub start: Point<f64>,
    pub end: Point<f64>,
    pub material: Material
}

/// A rotatable circle which is basically a circle with a material.
#[derive(Debug)]
pub struct Wheel {
    pub pos: Point<f64>,
    pub radius: f64,
    pub material: Material
}

/// A instance of a siege engine.
#[derive(Debug)]
pub struct Engine {
    pub pos: Point<f64>,
    pub parts: Vec<Part>
}

impl Engine {
    /// Place a empty siege engine at the given position with no parts.
    pub fn new(x: f64, y: f64) -> Self {
        Engine {
            pos: (x, y),
            parts: Vec::new()
        }
    }

    /// Render the graphics of the siege engine to a buffer.
    pub fn render_to_buffer(&self, dst: &mut [u32], dst_width: usize) {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
