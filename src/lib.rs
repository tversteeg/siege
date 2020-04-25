//! Procedurally generated siege engine generator.
//!
//! This library uses wave function collapse to generate an engine from a grid.

use anyhow::{anyhow, Error, Result};
use coord_2d::{Coord, Size};
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

const PATTERN_SIZE: i32 = 3;

/// A generated siege engine.
#[derive(Debug)]
pub struct Engine {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}

impl Engine {
    /// Render the engine as ASCII art.
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
/// This enum can be mapped to an ASCII character.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, FromPrimitive)]
pub enum Tile {
    /// `' '` ASCII: empty space.
    Empty,
    /// `'o'` ASCII: a single wheel.
    Wheel,
    /// `'-'` ASCII: a beam connecting the tile above and the tile below.
    HorizontalBeam,
    /// `'|'` ASCII: a beam connecting the tile left and the tile right.
    VerticalBeam,
    /// `'/'` ASCII: a beam connecting connecting left bottom to right top.
    DiagonalBeam1,
    /// `'\'` ASCII: a beam connecting connecting left top to right bottom.
    DiagonalBeam2,
    /// `'+'` ASCII: a cross section connecting beams.
    Cross,
    /// `'.'` ASCII: a wall tile in between beams.
    Wall,

    /// Edge tile, used internally.
    #[doc(hidden)]
    Edge = 255,
}

impl Tile {
    /// Create a tile from an ASCII character.
    pub fn from_ascii(ascii: char) -> Self {
        match ascii {
            ' ' => Tile::Empty,
            'o' => Tile::Wheel,
            '-' => Tile::HorizontalBeam,
            '|' => Tile::VerticalBeam,
            '/' => Tile::DiagonalBeam1,
            '\\' => Tile::DiagonalBeam2,
            '+' => Tile::Cross,
            '.' => Tile::Wall,
            _ => panic!("ascii character not supported"),
        }
    }

    /// Convert it to a single ASCII character.
    pub fn to_ascii(self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wheel => 'o',
            Tile::HorizontalBeam => '-',
            Tile::VerticalBeam => '|',
            Tile::DiagonalBeam1 => '/',
            Tile::DiagonalBeam2 => '\\',
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

    /// Use an ASCII art template.
    ///
    /// The symbols used are described in [`Tile`].
    ///
    /// [`Tile`]: enum.Tile.html
    pub fn from_ascii<S>(ascii: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let mut height = 0;
        let mut tiles = vec![];

        // Iterate over all the lines in the text
        for line in ascii.as_ref().lines() {
            // Map the text characters to tiles in a vector
            let line_tiles = line
                .chars()
                .map(|ch| Tile::from_ascii(ch))
                .collect::<Vec<_>>();
            tiles.push(line_tiles);

            height += 1;
        }

        // Get the biggest line width
        let width = tiles
            .iter()
            .max_by_key(|line_tiles| line_tiles.len())
            .ok_or_else(|| anyhow!("could not find maximum in array"))?
            .len();

        let tiles = tiles
            .into_iter()
            .map(|mut line_tiles| {
                if width > line_tiles.len() {
                    // Fill the tiles with smaller widths with empty tiles
                    line_tiles.resize_with(width, || Tile::Empty);
                }

                line_tiles
            })
            .flatten()
            .collect::<Vec<_>>();

        Self::from_array(width as u32, height, &tiles[..])
    }

    /// Use a template from a path pointing to an ASCII file.
    ///
    /// The symbols used are described in [`Tile`].
    ///
    /// [`Tile`]: enum.Tile.html
    pub fn from_ascii_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        // Read the file
        let mut file_contents = String::new();
        File::open(path)?.read_to_string(&mut file_contents)?;

        // Parse it
        Self::from_ascii(file_contents)
    }

    /// Generate a 2D grid with the output dimensions passed of a randomly generated siege engine.
    pub fn generate_skeleton<R>(
        &self,
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
        Self::from_ascii(include_str!("default.ascii")).unwrap()
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
