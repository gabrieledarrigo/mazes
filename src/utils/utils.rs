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
pub fn random_cell(grid: &impl BaseGrid) -> GridCell {
    let mut rng = rand::thread_rng();
    let row = rng.gen_range(0..grid.rows());
    let column = rng.gen_range(0..grid.columns());

    grid.cell(row, column).unwrap().clone()
}

/// Returns a random neighbor of the given cell in the given grid.
///
/// # Arguments
///
/// * `grid` - The grid in which the cell is located.
/// * `cell` - The cell for which to find a random neighbor.
///
/// # Returns
///
/// The randomly selected neighbor `GridCell`.
pub fn random_neighbor(grid: &impl BaseGrid, cell: GridCell) -> GridCell {
    let mut rng = rand::thread_rng();

    let neighbors = cell.borrow_mut().neighbors();
    let index = rng.gen_range(0..neighbors.len());
    let (row, column) = neighbors[index];

    grid.cell(row, column).unwrap().clone()
}
