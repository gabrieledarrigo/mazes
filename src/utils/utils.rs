use crate::grids::base_grid::{BaseGrid, GridCell};
use rand::Rng;

pub fn random_cell(grid: &impl BaseGrid) -> GridCell {
    let mut rng = rand::thread_rng();
    let row = rng.gen_range(0..grid.rows());
    let column = rng.gen_range(0..grid.columns());

    grid.cell(row, column).unwrap().clone()
}

pub fn random_neighbor(grid: &impl BaseGrid, cell: GridCell) -> GridCell {
    let mut rng = rand::thread_rng();

    let neighbors = cell.borrow_mut().neighbors();
    let index = rng.gen_range(0..neighbors.len());
    let (row, column) = neighbors[index];

    grid.cell(row, column).unwrap().clone()
}
