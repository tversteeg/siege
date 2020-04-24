use anyhow::Result;
use coord_2d::Size;
use grid_2d::Grid;
use std::{num::NonZeroU32, path::Path};
use wfc::{overlapping::OverlappingPatterns, GlobalStats, RunOwn};

type Index = u8;

/// The siege engine generator.
pub struct Generator {
    //pub grid: Grid<Index>,
//pub overlapping_pattern: OverlappingPatterns<Index>,
}

impl Generator {
    /// Use a template from a CSV.
    pub fn from_csv<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        unimplemented!();
    }

    /// Generate a siege engine.
    pub fn run(width: u32, height: u32) {
        let size = Size::new(width, height);
        let grid = Grid::<Index>::new_default(size);
        let overlapping_patterns =
            OverlappingPatterns::new_original_orientation(grid, NonZeroU32::new(1).unwrap());

        //let run = RunOwn::new(Size::new(width, height));
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self {}
    }
}
