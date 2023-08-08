//! # `smoothlife`
//!

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![deny(rust_2018_idioms, unsafe_code)]

use std::borrow::Cow;

#[cfg(feature = "dhat")]
#[global_allocator]
pub static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Debug)]
pub struct Grid<const W: usize, const H: usize> {
    grid: [[f32; W]; H],
    alpha: f32,
    birdth_rate: (f32, f32),
    death_rate: (f32, f32),
}

const LEVELS: &[char] = &[' ', '-', '=', '*', 'c', 'o', 'a', 'A', '@', '#'];
const LEVELS_LEN: usize = LEVELS.len();

impl<const W: usize, const H: usize> Default for Grid<W, H> {
    fn default() -> Self {
        Grid {
            grid: [[0.; W]; H],
            alpha: 5.,
            birdth_rate: (0.5, 0.5),
            death_rate: (0.5, 0.5),
        }
    }
}

impl<const W: usize, const H: usize> std::fmt::Display for Grid<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.iter().try_for_each(|row| {
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

    #[inline(always)]
    fn sigma1(x: f32, a: f32, alpha: f32) -> f32 {
        1. / 1. + (-(x - a) * 4. / alpha).exp()
    }

    #[inline(always)]
    fn sigma2(x: f32, a: f32, b: f32, alpha: f32) -> f32 {
        Self::sigma1(x, a, alpha) * (1. - Self::sigma1(x, b, alpha))
    }

    #[inline(always)]
    fn sigmaM(x: f32, y: f32, m: f32, alpha: f32) -> f32 {
        x * (1. - Self::sigma1(m, 0.5, alpha)) + y * Self::sigma1(m, 0.5, alpha)
    }

    #[must_use]
    pub fn into_rand_state(mut self) -> Self {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        self.grid.iter_mut().for_each(|row| {
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
