mod algorithms;
mod grids;

use algorithms::{binary_tree::*, sidewinder::*};
use grids::distance_grid::DistanceGrid;
use grids::grid::Grid;

use crate::algorithms::aldous_broder::AldousBroder;
use crate::grids::base_grid::BaseGrid;

fn main() {
    println!("Binary tree");
    let mut grid = Grid::new(5, 5);
    BinaryTree::on(&mut grid);
    println!("{}", grid.display());

    println!("Sidewinder");
    let mut grid_1 = Grid::new(5, 5);
    Sidewinder::on(&mut grid_1);
    println!("{}", grid_1.display());

    println!("With distances");
    let mut distance_grid = DistanceGrid::new(5, 5);
    BinaryTree::on(&mut distance_grid);
    println!("{}", distance_grid.display());

    println!("With path from north west to south east");
    println!(
        "{}",
        distance_grid.display_path_to(distance_grid.cell(4, 0).unwrap().clone())
    );

    println!("With colors");
    println!("{}", distance_grid.display_with_color());

    println!("Aldous Brother");
    let mut grid_2 = Grid::new(5, 5);
    AldousBroder::on(&mut grid_2);
    println!("{}", grid_2.display());
}
