mod binary_tree;
mod cell;
mod grid;

use binary_tree::BinaryTree;
use grid::Grid;

fn main() {
    let mut grid = Grid::new(4, 4);
    BinaryTree::on(&mut grid);

    println!("{}", grid);
}
