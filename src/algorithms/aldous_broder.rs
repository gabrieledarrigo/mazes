use rand::Rng;

use crate::grids::base_grid::{BaseGrid, GridCell};

pub struct AldousBroder {}

impl AldousBroder {
    pub fn on(grid: &mut impl BaseGrid) {
        let mut cell = Self::random_cell(grid);
        let mut unvisited = (grid.rows() * grid.columns()) - 1;

        while unvisited > 0 {
            let neighbor = Self::random_neighbor(grid, cell.clone());

            if neighbor.borrow_mut().links().is_empty() {
                cell.borrow_mut().link(neighbor.clone());
                unvisited -= 1;
            }

            cell = neighbor;
        }
    }

    fn random_cell(grid: &mut impl BaseGrid) -> GridCell {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..grid.rows());
        let column = rng.gen_range(0..grid.columns());

        grid.cell(row, column).unwrap().clone()
    }

    fn random_neighbor(grid: &mut impl BaseGrid, cell: GridCell) -> GridCell {
        let mut rng = rand::thread_rng();

        let neighbors = cell.borrow_mut().neighbors();
        let index = rng.gen_range(0..neighbors.len());
        let (row, column) = neighbors[index];

        grid.cell(row, column).unwrap().clone()
    }
}
