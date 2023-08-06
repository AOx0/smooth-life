use smoothlife::*;

fn main() {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();

    let grid: Grid<10, 10> = Grid::new().into_rand_state();

    println!("{:}", grid);
}
