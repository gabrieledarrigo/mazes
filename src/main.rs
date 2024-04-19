mod algorithms;
mod grids;
mod utils;

use algorithms::{
    aldous_broder::AldousBroder, binary_tree::BinaryTree, hunt_and_kill::HuntAndKill,
    recursive_backtracker::RecursiveBacktracker, sidewinder::Sidewinder, wilsons::Wilsons,
};
use grids::{base_grid::BaseGrid, distance_grid::DistanceGrid, grid::Grid};

fn main() {
    println!("Binary tree");
    let mut grid = Grid::new(6, 6);
    BinaryTree::on(&mut grid);
    println!("{}", grid.display());

    println!("Sidewinder");
    let mut grid = Grid::new(6, 6);
    Sidewinder::on(&mut grid);
    println!("{}", grid.display());

    println!("With distances");
    let mut grid = DistanceGrid::new(6, 6);
    BinaryTree::on(&mut grid);
    println!("{}", grid.display());

    println!("With path from north west to south east");
    println!("{}", grid.display_path_to(grid.cell(5, 0).unwrap().clone()));

    println!("With colors");
    println!("{}", grid.display_with_color());

    println!("Aldous Brother");
    let mut grid = DistanceGrid::new(6, 6);
    AldousBroder::on(&mut grid);
    println!("{}", grid.display_with_color());

    println!("Wilsons");
    let mut grid = Grid::new(6, 6);
    Wilsons::on(&mut grid);
    println!("{}", grid.display());

    println!("Hunt And Kill");
    let mut grid = Grid::new(6, 6);
    HuntAndKill::on(&mut grid);
    println!("{}", grid.display());

    println!("Recursive Backtracker");
    let mut grid = Grid::new(6, 6);
    RecursiveBacktracker::on(&mut grid);
    println!("{}", grid.display());
}
