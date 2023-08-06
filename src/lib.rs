//! # `smoothlife`
//!

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![deny(rust_2018_idioms, unsafe_code)]

#[cfg(feature = "dhat")]
#[global_allocator]
pub static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Debug)]
pub struct Grid<const W: usize, const H: usize>([[f32; W]; H]);

const LEVELS: &[char] = &[' ', '-', '=', '*', 'c', 'o', 'a', 'A', '@', '#'];
const LEVELS_LEN: usize = LEVELS.len();

impl<const W: usize, const H: usize> Default for Grid<W, H> {
    fn default() -> Self {
        Grid([[0.; W]; H])
    }
}

impl<const W: usize, const H: usize> std::fmt::Display for Grid<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|row| {
            row.iter().try_for_each(|col| {
                let i = (col * (LEVELS_LEN as f32 - 1.)).round() as usize % LEVELS_LEN;
                write!(f, "{:} ", LEVELS[i])
            })?;
            write!(f, "\n")
        })
    }
}

impl<const W: usize, const H: usize> Grid<W, H> {
    #[must_use]
    pub fn new() -> Self {
        Grid::default()
    }

    #[must_use]
    pub fn into_rand_state(mut self) -> Self {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        self.0.iter_mut().for_each(|row| {
            row.iter_mut()
                .for_each(|col| *col = rng.gen_range((0.)..=(1.)))
        });
        self
    }
}

#[must_use]
pub fn test() -> &'static str {
    "Hello World"
}
