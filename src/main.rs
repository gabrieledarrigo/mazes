mod algorithms;
mod grids;

use algorithms::{binary_tree::*, sidewinder::*};
use grids::distance_grid::DistanceGrid;
use grids::grid::Grid;

fn main() {
    let mut grid = Grid::new(4, 4);
    BinaryTree::on(&mut grid);
    println!("{}", grid.display());

    let mut grid_1 = Grid::new(4, 4);
    Sidewinder::on(&mut grid_1);
    println!("{}", grid_1.display());

    let mut distance_grid = DistanceGrid::new(4, 4);
    BinaryTree::on(&mut distance_grid);
    println!("{}", distance_grid.display());
}
