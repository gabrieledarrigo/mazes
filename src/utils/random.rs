use crate::grids::base_grid::{BaseGrid, GridCell};
use rand::Rng;

/// Returns a random cell from the given grid.
///
/// # Arguments
///
/// * `grid` - The grid from which to select a random cell.
///
/// # Returns
///
/// The randomly selected `GridCell`.
pub fn random_cell(grid: &dyn BaseGrid) -> GridCell {
    let mut rng = rand::thread_rng();
    let row = rng.gen_range(0..grid.rows());
    let column = rng.gen_range(0..grid.columns());

    grid.cell(row, column).unwrap().clone()
}

/// Returns a random neighbor cell from the given grid and a list of neighbors.
///
/// # Arguments
///
/// * `grid` - The grid from which to select a random neighbor cell.
/// * `neighbors` - A list of coordinates representing the neighbors of a cell.
///
/// # Returns
///
/// The randomly selected neighbor `GridCell`.
pub fn random_neighbor(grid: &dyn BaseGrid, neighbors: &Vec<(i32, i32)>) -> GridCell {
    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..neighbors.len());
    let (row, column) = neighbors[index];

    grid.cell(row, column).unwrap().clone()
}
