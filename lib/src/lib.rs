extern crate line_drawing;

use line_drawing::{Point, Bresenham, BresenhamCircle};

/// The material that the parts are made of.
/// This determines how they get rendered and their strength.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Material {
    Wood,
    Metal,
    Rope
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

impl Beam {
    pub fn new(start: (i32, i32), end: (i32, i32), material: Material) -> Self {
        Beam {
            material,

            start: (start.0 as f64, start.1 as f64),
            end: (end.0 as f64, end.1 as f64),
        }
    }

    pub fn draw(&self, dst: &mut [u32], dst_width: usize) {
        let color = match self.material {
            Material::Wood => 0x8F563B,
            Material::Metal => 0x696A6A,
            Material::Rope => 0xD9A066,
        };

        let start = (self.start.0 as i32, self.start.1 as i32);
        let end = (self.end.0 as i32, self.end.1 as i32);
        for (x, y) in Bresenham::new(start, end) {
            dst[x as usize + y as usize * dst_width] = color;
        }
    }
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
        for part in self.parts.iter() {
            if let &Part::Beam(ref beam) = part {
                beam.draw(dst, dst_width);
            }
        }
    }

    /// Add a beam with the material.
    pub fn add_beam(&mut self, start: (i32, i32), end: (i32, i32), material: Material) {
        self.parts.push(Part::Beam(Beam::new(start, end, material)));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
