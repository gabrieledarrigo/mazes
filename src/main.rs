mod binary_tree;
mod cell;
mod grid;
mod sidewinder;

use binary_tree::BinaryTree;
use grid::Grid;

use crate::sidewinder::Sidewinder;

fn main() {
    let mut grid = Grid::new(3, 3);
    BinaryTree::on(&mut grid);
    println!("{}", grid);

    let mut grid_1 = Grid::new(3, 3);
    Sidewinder::on(&mut grid_1);
    println!("{}", grid_1);
}
