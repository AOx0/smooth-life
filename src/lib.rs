//! # `smoothlife`
//!

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![deny(rust_2018_idioms, unsafe_code)]

#[cfg(feature = "dhat")]
#[global_allocator]
pub static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Debug)]
pub struct Grid<const W: usize, const H: usize> {
    grid: [[f32; W]; H],
    grid_diff: [[f32; W]; H],
    alpha: (f32, f32),
    birdth_rate: (f32, f32),
    death_rate: (f32, f32),
    ra: usize,
    dt: f32,
    // coords: (usize, usize),
}

const LEVELS: &[char] = &[' ', '.', '-', '=', 'c', 'o', 'a', 'A', '@', '#'];
const LEVELS_LEN: usize = LEVELS.len();

impl<const W: usize, const H: usize> Default for Grid<W, H> {
    fn default() -> Self {
        Grid {
            grid: [[0.; W]; H],
            grid_diff: [[0.; W]; H],
            alpha: (0.028, 0.147),
            birdth_rate: (0.278, 0.365),
            death_rate: (0.267, 0.445),
            ra: 21,
            dt: 0.05,
            // coords: (0, 0),
        }
    }
}

impl<const W: usize, const H: usize> std::fmt::Display for Grid<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.iter().enumerate().try_for_each(|(r, row)| {
            row.iter().try_for_each(|col| {
                let i = (col * (LEVELS_LEN as f32 - 1.)).round() as usize % LEVELS_LEN;
                write!(f, "{0}{0}", LEVELS[i])
            })?;
            if r - 1 != H {
                writeln!(f)
            } else {
                Ok(())
            }
        })
    }
}

impl<const W: usize, const H: usize> Grid<W, H> {
    #[must_use]
    pub fn into_rand_state(mut self) -> Self {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        (0..H / 3).for_each(|r| {
            (0..W / 3).for_each(|c| {
                let x = c + W / 2 - (W / 3) / 2;
                let y = r + H / 2 - (H / 3) / 2;
                self.grid[y][x] = rng.gen_range((0.2)..=0.8)
                // *col = rng.gen_range((0.)..=1.) * rng.gen_range::<f32, _>((0.)..=1.).round()
            })
        });
        self
    }

    #[must_use]
    pub fn new() -> Self {
        Grid::default()
    }

    #[inline(always)]
    fn s(&self, m: f32, n: f32) -> f32 {
        let (an, am) = self.alpha;
        Self::sigma_n(
            n,
            Self::sigma_m(self.birdth_rate.0, self.death_rate.0, m, am),
            Self::sigma_m(self.birdth_rate.1, self.death_rate.1, m, am),
            an,
        )
    }

    #[inline(always)]
    fn sigma(x: f32, a: f32, alpha: f32) -> f32 {
        1. / (1. + (-(x - a) * 4. / alpha).exp())
    }

    #[inline(always)]
    fn sigma_m(x: f32, y: f32, m: f32, alpha: f32) -> f32 {
        x * (1. - Self::sigma(m, 0.5, alpha)) + y * Self::sigma(m, 0.5, alpha)
    }

    #[inline(always)]
    fn sigma_n(x: f32, a: f32, b: f32, alpha: f32) -> f32 {
        Self::sigma(x, a, alpha) * (1. - Self::sigma(x, b, alpha))
    }

    pub fn step(&mut self) {
        for cy in 0..W {
            for cx in 0..H {
                #[allow(non_snake_case)]
                let (mut M, mut N) = (0, 0);
                let (mut m, mut n) = (0., 0.);

                let r = self.ra;
                let rf = self.ra as f32;
                let ri = self.ra as f32 / 3.;

                // println!("({}, {}) r:{}", cx, cy, r);

                for dy in (-(r as i64 - 1))..(r as i64 - 1) {
                    let row = self.grid[(dy + cy as i64).rem_euclid(H as i64) as usize];
                    for dx in (-(r as i64 - 1))..(r as i64 - 1) {
                        let col = row[(dx + cx as i64).rem_euclid(W as i64) as usize];
                        // let i = (col * (LEVELS_LEN as f32 - 1.)).round() as usize % LEVELS_LEN;
                        // print!("{:} ", LEVELS[i]);
                        let sum = (dx * dx + dy * dy) as f32;
                        m += col * (sum <= ri * ri) as i32 as f32;
                        n += col * (sum <= rf * rf) as i32 as f32;
                        M += (sum <= ri * ri) as i32;
                        N += (sum <= rf * rf) as i32;
                    }
                    // println!();
                }

                m /= M as f32;
                n /= N as f32;

                self.grid_diff[cy][cx] = 2. * self.s(n, m) - 1.;
            }
        }

        for h in 0..H {
            self.grid[h]
                .iter_mut()
                .zip(self.grid_diff[h].iter())
                .for_each(|(row, diff)| {
                    *row += diff * self.dt;
                    if *row < 0. {
                        *row = 0.
                    }
                    if *row > 1. {
                        *row = 1.
                    }
                })
        }
    }

    pub fn with_birdth_rate(mut self, birdth_rate: (f32, f32)) -> Self {
        self.birdth_rate = birdth_rate;
        self
    }

    pub fn with_death_rate(mut self, death_rate: (f32, f32)) -> Self {
        self.death_rate = death_rate;
        self
    }

    pub fn with_dt(mut self, dt: f32) -> Self {
        self.dt = dt;
        self
    }

    pub fn with_ra(mut self, ra: usize) -> Self {
        self.ra = ra;
        self
    }
}

#[must_use]
pub fn test() -> &'static str {
    "Hello World"
}
