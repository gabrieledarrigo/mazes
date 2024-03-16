mod algorithms;
mod grids;

use algorithms::{binary_tree::*, sidewinder::*};
use grids::distance_grid::DistanceGrid;
use grids::grid::Grid;

use crate::grids::base_grid::BaseGrid;

fn main() {
    let mut grid = Grid::new(5, 5);
    BinaryTree::on(&mut grid);
    println!("{}", grid.display());

    let mut grid_1 = Grid::new(5, 5);
    Sidewinder::on(&mut grid_1);
    println!("{}", grid_1.display());

    let mut distance_grid = DistanceGrid::new(5, 5);
    BinaryTree::on(&mut distance_grid);
    println!("{}", distance_grid.display());
    println!("{}", distance_grid.display_with_color());
    println!(
        "{}",
        distance_grid.display_path_to(distance_grid.cell(4, 0).unwrap().clone())
    );
}
