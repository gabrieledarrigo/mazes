use crate::{grids::base_grid::BaseGrid, utils::utils::*};
use rand::Rng;

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

            let mut cell = random_cell(grid);
            path.push(cell.clone());

            while unvisited.contains(&&cell) {
                cell = random_neighbor(grid, cell.clone());

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grids::grid::Grid;

    #[test]
    fn test_wilsons_on() {
        let grid = Grid::new(3, 3);
        Wilsons::on(&grid);

        // Assert that all cells are linked
        for cell in grid.iter() {
            assert!(cell.borrow().links().len() > 0);
        }
    }
}
