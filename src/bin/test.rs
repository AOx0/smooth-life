use smoothlife::*;

fn main() {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();

    let mut grid: Grid<150, 150> = Grid::new()
        .with_birdth_rate((0.258, 0.355))
        .with_death_rate((20., 130.))
        .with_dt(0.15)
        .with_ra(10)
        .into_rand_state();

    // more random 2
    //
    // let mut grid: Grid<150, 150> = Grid::new()
    //     .with_birdth_rate((0.258, 0.355))
    //     .with_death_rate((10., 15.))
    //     .with_dt(0.05)
    //     .with_ra(20)
    //     .into_rand_state();

    // more random
    //
    // let mut grid: Grid<150, 150> = Grid::new()
    //     .with_birdth_rate((0.258, 0.355))
    //     .with_death_rate((10., 10.))
    //     .with_dt(0.05)
    //     .with_ra(20)
    //     .into_rand_state();

    // Debian?
    //
    // let mut grid: Grid<100, 100> = Grid::new()
    //     .with_birdth_rate((0.128, 0.265))
    //     .with_death_rate((0.667, 0.745))
    //     .with_dt(0.05)
    //     .with_ra(40)
    //     .into_rand_state();

    // Football
    //
    // let mut grid: Grid<100, 100> = Grid::new()
    //     .with_birdth_rate((0.128, 0.265))
    //     .with_death_rate((0.667, 0.745))
    //     .with_dt(0.05)
    //     .with_ra(30)
    //     .into_rand_state();

    // tscoding
    //
    // let mut grid: Grid<100, 100> = Grid::new()
    //     .with_birdth_rate((0.278, 0.365))
    //     .with_death_rate((0.267, 0.445))
    //     .with_dt(0.05)
    //     .with_ra(11)
    //     .into_rand_state();

    // Lots of cirlces
    //
    // let mut grid: Grid<100, 100> = Grid::new()
    //     .with_birdth_rate((0.178, 0.265))
    //     .with_death_rate((0.467, 0.545))
    //     .with_dt(0.05)
    //     .with_ra(21)
    //     .into_rand_state();

    loop {
        // print!("{esc}c", esc = 27 as char);
        print!("{:}", grid);
        grid.step();
    }
}
