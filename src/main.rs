mod algorithms;
mod grids;

use algorithms::{aldous_broder::AldousBroder, binary_tree::BinaryTree, sidewinder::Sidewinder};
use grids::{base_grid::BaseGrid, distance_grid::DistanceGrid, grid::Grid};

fn main() {
    println!("Binary tree");
    let mut grid = Grid::new(6, 6);
    BinaryTree::on(&mut grid);
    println!("{}", grid.display());

    println!("Sidewinder");
    let mut grid_1 = Grid::new(6, 6);
    Sidewinder::on(&mut grid_1);
    println!("{}", grid_1.display());

    println!("With distances");
    let mut distance_grid = DistanceGrid::new(6, 6);
    BinaryTree::on(&mut distance_grid);
    println!("{}", distance_grid.display());

    println!("With path from north west to south east");
    println!(
        "{}",
        distance_grid.display_path_to(distance_grid.cell(5, 0).unwrap().clone())
    );

    println!("With colors");
    println!("{}", distance_grid.display_with_color());

    println!("Aldous Brother");
    let mut grid_2 = DistanceGrid::new(6, 6);
    AldousBroder::on(&mut grid_2);
    println!("{}", grid_2.display_with_color());
}
