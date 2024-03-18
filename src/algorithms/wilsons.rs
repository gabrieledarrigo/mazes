use rand::Rng;

use crate::grids::base_grid::{BaseGrid, GridCell};

pub struct Wilsons {}

impl Wilsons {
    pub fn on(grid: &impl BaseGrid) {
        let mut unvisited = vec![];

        for cell in grid.iter() {
            unvisited.push(cell);
        }

        let mut rng = rand::thread_rng();
        let first = rng.gen_range(0..unvisited.len());
        unvisited.remove(first);

        while !unvisited.is_empty() {
            let mut path = vec![];

            let mut cell = Self::random_cell(grid);
            path.push(cell.clone());

            while unvisited.contains(&&cell) {
                cell = Self::random_neighbor(grid, cell.clone());

                let position = path.iter().position(|c| {
                    c.borrow().to_row_and_column() == cell.borrow().to_row_and_column()
                });

                if let Some(position) = position {
                    path.truncate(position + 1);
                } else {
                    path.push(cell.clone());
                }
            }

            let mut visited = vec![];
            for index in 0..path.len() - 1 {
                path[index].borrow_mut().link(path[index + 1].clone());
                visited.push(path[index].clone());
            }

            unvisited.retain(|cell| !visited.contains(cell));
        }
    }

    fn random_cell(grid: &impl BaseGrid) -> GridCell {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..grid.rows());
        let column = rng.gen_range(0..grid.columns());

        grid.cell(row, column).unwrap().clone()
    }

    fn random_neighbor(grid: &impl BaseGrid, cell: GridCell) -> GridCell {
        let mut rng = rand::thread_rng();

        let neighbors = cell.borrow_mut().neighbors();
        let index = rng.gen_range(0..neighbors.len());
        let (row, column) = neighbors[index];

        grid.cell(row, column).unwrap().clone()
    }
}
