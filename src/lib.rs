//! Procedurally generated siege engine generator.
//!
//! This library uses wave function collapse to generate an engine from a grid.

use anyhow::{anyhow, Error, Result};
use coord_2d::{Coord, Size};
use csv_core::{ReadFieldResult, Reader};
use grid_2d::Grid;
use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::Rng;
use std::{
    fs::File,
    io::Read,
    num::NonZeroU32,
    path::Path,
    str::{self, FromStr},
};
use wfc::{
    overlapping::OverlappingPatterns, retry::NumTimes, ForbidInterface, ForbidPattern, PatternId,
    RunOwn, Wrap,
};

const PATTERN_SIZE: i32 = 2;

/// A generated siege engine.
#[derive(Debug)]
pub struct Engine {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}

impl Engine {
    /// Render the engine as ascii art.
    pub fn to_ascii(&self) -> String {
        self.tiles
            .iter()
            .chunks(self.width as usize)
            .into_iter()
            .map(|row| row.map(|tile| tile.to_ascii()).collect::<String>())
            .join("\n")
    }
}

/// Grid section of the siege engine.
///
/// This enum can be mapped to a number to be used in the CSV.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, FromPrimitive)]
pub enum Tile {
    /// `0`: empty space.
    Empty = 0,
    /// `1`: a single wheel.
    Wheel = 1,
    /// `2`: a beam connecting the tile above and the tile below.
    HorizontalBeam = 2,
    /// `3`: a beam connecting the tile left and the tile right.
    VerticalBeam = 3,
    /// `4`: a cross section connecting beams.
    Cross = 4,
    /// `5`: a wall tile in between beams.
    Wall = 5,

    /// Edge tile, used internally.
    #[doc(hidden)]
    Edge = 255,
}

impl Tile {
    /// Convert it to a single ascii character.
    pub fn to_ascii(self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wheel => 'o',
            Tile::HorizontalBeam => '-',
            Tile::VerticalBeam => '|',
            Tile::Cross => '+',
            Tile::Wall => '.',
            Tile::Edge => panic!("this tile should've been removed from the output"),
        }
    }
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First attempt to convert it from a number
        Ok(match s.parse::<u8>() {
            Ok(num) => match FromPrimitive::from_u8(num) {
                Some(tile) => tile,
                None => {
                    return Err(anyhow!("no tile with this number can be mapped"));
                }
            },
            Err(err) => {
                // TODO allow usage of letters and names and handle them here
                return Err(anyhow!("{}", err));
            }
        })
    }
}

/// The siege engine generator.
pub struct Generator {
    pub grid: Grid<Tile>,
    pub overlapping_patterns: OverlappingPatterns<Tile>,
}

impl Generator {
    /// Use a template from an array of numbers.
    pub fn from_array(width: u32, height: u32, map: &[Tile]) -> Result<Self> {
        if map.len() != (width * height) as usize {
            return Err(anyhow!("array doesn't match width & height"));
        }

        // Add 2 to the dimensions to pad all sides with empty tiles later
        let size = Size::new(width + 2, height + 2);

        // Convert the array to a 2d grid
        let grid = Grid::new_fn(size, |coord| {
            if coord_is_edge(coord, size.width() as i32, size.height() as i32) {
                // Pad the with empty tiles
                Tile::Edge
            } else {
                map[((coord.y - 1) * (width as i32) + (coord.x - 1)) as usize]
            }
        });

        let overlapping_patterns = OverlappingPatterns::new_original_orientation(
            grid.clone(),
            NonZeroU32::new(PATTERN_SIZE as u32).unwrap(),
        );

        Ok(Self {
            grid,
            overlapping_patterns,
        })
    }

    /// Use a template from CSV string.
    pub fn from_csv<S>(csv: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let mut reader = Reader::new();
        let mut bytes = csv.as_ref().as_bytes();
        let mut index = 0;
        let mut height = 0;

        let mut fields = vec![];

        // Parse the CSV file
        loop {
            let (result, nin, nread) = reader.read_field(bytes, &mut [0; 1024]);
            let field = &bytes[0..nread];
            bytes = &bytes[nin..];
            match result {
                ReadFieldResult::InputEmpty => {}
                ReadFieldResult::OutputFull => Err(anyhow!("csv field too large"))?,
                ReadFieldResult::Field { record_end } => {
                    // Convert the string to an integer
                    let field_as_number = str::from_utf8(field)?.parse::<Tile>()?;
                    fields.push(field_as_number);

                    // Keep track of the size of the grid
                    index += 1;
                    if record_end {
                        height += 1;
                    }
                }
                ReadFieldResult::End => break,
            }
        }

        Self::from_array(index / height, height, &fields[..])
    }

    /// Use a template from a path pointing to a CSV file.
    pub fn from_csv_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        // Read the file
        let mut file_contents = String::new();
        File::open(path)?.read_to_string(&mut file_contents)?;

        // Parse it
        Self::from_csv(file_contents)
    }

    /// Generate a 2D grid with the output dimensions passed of a randomly generated siege engine.
    pub fn generate_skeleton<R>(
        &mut self,
        output_width: u32,
        output_height: u32,
        retry_times: usize,
        rng: &mut R,
    ) -> Option<Engine>
    where
        R: Rng,
    {
        // Construct the WFC runner
        let global_stats = self.overlapping_patterns.global_stats();
        let run = RunOwn::new_forbid(
            Size::new(output_width + 2, output_height + 2),
            &global_stats,
            self.force_border_forbid(),
            rng,
        );

        // Attempt to collapse with a set amount of retries
        let wave = run.collapse_retrying(NumTimes(retry_times), rng).ok()?;
        let grid = wave.grid();

        // Map the result to indices of the original vector
        let tiles = grid
            .iter()
            .map(|cell| {
                // Get the original index
                let pattern_id = cell.chosen_pattern_id().unwrap();
                self.overlapping_patterns
                    .pattern_top_left_value(pattern_id)
                    .clone()
            })
            // Remove all edge tiles
            .filter(|tile| *tile != Tile::Edge)
            .collect();

        Some(Engine {
            tiles,
            width: output_width,
            height: output_height,
        })
    }

    /// Create the forbid pattern.
    fn force_border_forbid(&self) -> ForceBorderForbid {
        let size = self.grid.size();
        let width = size.width() as i32;
        let height = size.height() as i32;

        let overlapping_grid = self.overlapping_patterns.id_grid_original_orientation();

        // Always let the corner be empty tiles
        let top_left = *overlapping_grid.get_checked(Coord::new(0, 0));
        let bot_left = *overlapping_grid.get_checked(Coord::new(0, height - 1));
        let top_right = *overlapping_grid.get_checked(Coord::new(width - 1, 0));
        let bot_right = *overlapping_grid.get_checked(Coord::new(width - 1, height - 1));

        // Get all the patterns containing empty tiles
        let mut pattern_ids = Vec::with_capacity(width as usize * 2 + height as usize * 2);
        for x in 0..width {
            pattern_ids.push(overlapping_grid.get_checked(Coord::new(x, 0)).clone());
            pattern_ids.push(
                overlapping_grid
                    .get_checked(Coord::new(x, height - 1))
                    .clone(),
            );
        }
        for y in 0..height {
            pattern_ids.push(overlapping_grid.get_checked(Coord::new(0, y)).clone());
            pattern_ids.push(
                overlapping_grid
                    .get_checked(Coord::new(width - 1, y))
                    .clone(),
            );
        }

        ForceBorderForbid {
            pattern_ids,
            top_left,
            bot_left,
            top_right,
            bot_right,
        }
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::from_csv(include_str!("default.csv")).unwrap()
    }
}

/// Ensures that the the edges will always be filled with empty tiles.
#[derive(Clone)]
struct ForceBorderForbid {
    pattern_ids: Vec<PatternId>,
    top_left: PatternId,
    bot_left: PatternId,
    top_right: PatternId,
    bot_right: PatternId,
}

impl ForbidPattern for ForceBorderForbid {
    fn forbid<W: Wrap, R: Rng>(&mut self, fi: &mut ForbidInterface<W>, rng: &mut R) {
        let output_size = fi.wave_size();
        let width = output_size.width() as i32;
        let height = output_size.height() as i32;

        fi.forbid_all_patterns_except(Coord::new(0, 0), self.top_left, rng)
            .unwrap();
        fi.forbid_all_patterns_except(Coord::new(0, height - 1), self.bot_left, rng)
            .unwrap();
        fi.forbid_all_patterns_except(Coord::new(width - 1, 0), self.top_right, rng)
            .unwrap();
        fi.forbid_all_patterns_except(Coord::new(width - 1, height - 1), self.bot_right, rng)
            .unwrap();

        for x in 0..width {
            for y in 0..height {
                let coord = Coord::new(x, y);
                if !coord_is_edge(coord, width, height) {
                    // The inside is never allowed to be an edge
                    self.pattern_ids
                        .iter()
                        .for_each(|pattern_id| fi.forbid_pattern(coord, *pattern_id, rng).unwrap());
                }
            }
        }
    }
}

fn coord_is_edge(coord: Coord, width: i32, height: i32) -> bool {
    coord.x == 0 || coord.x == width - 1 || coord.y == 0 || coord.y == height - 1
}

#[cfg(test)]
mod tests {
    use crate::*;
    use anyhow::Result;

    #[test]
    fn default_test() -> Result<()> {
        let result = Generator::default().generate_skeleton(10, 10, 100, &mut rand::thread_rng());
        assert!(result.is_some());

        Ok(())
    }
}
