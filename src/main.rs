mod algorithms;
mod grids;

use algorithms::{binary_tree::*, sidewinder::*};
use grids::grid::Grid;

fn main() {
    let mut grid = Grid::new(3, 3);
    BinaryTree::on(&mut grid);
    println!("{}", grid);

    let mut grid_1 = Grid::new(3, 3);
    Sidewinder::on(&mut grid_1);
    println!("{}", grid_1);
}
