use super::On;
use crate::{grids::base_grid::BaseGrid, utils::utils::*};

/// The Aldous-Broder algorithm for generating a maze.
pub struct AldousBroder {}

impl AldousBroder {
    pub fn new() -> Self {
        Self {}
    }
}

impl On for AldousBroder {
    /// Generates a maze using the Aldous-Broder algorithm on the given grid.
    ///
    /// # Arguments
    ///
    /// * `grid` - The grid on which to generate the maze.
    fn on(&self, grid: &mut dyn BaseGrid) {
        let mut cell = random_cell(grid);
        let mut unvisited = (grid.rows() * grid.columns()) - 1;

        while unvisited > 0 {
            let neighbor = random_neighbor(grid, cell.borrow().neighbors());

            if neighbor.borrow_mut().links().is_empty() {
                cell.borrow_mut().link(neighbor.clone());
                unvisited -= 1;
            }

            cell = neighbor;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grids::{base_grid::WithRowsAndColumns, grid::Grid};

    #[test]
    fn test_aldoous_broder() {
        let mut grid = Grid::new(5, 5);
        let aldous_broder = AldousBroder::new();

        aldous_broder.on(&mut grid);

        assert_eq!(grid.rows(), 5);
        assert_eq!(grid.columns(), 5);

        for cell in grid.iter() {
            assert!(!cell.borrow().links().is_empty());
        }
    }
}
