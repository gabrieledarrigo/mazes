use super::On;
use crate::{
    grids::base_grid::{BaseGrid, GridCell},
    utils::random::{random_cell, random_neighbor},
};

/// The `RecursiveBacktracker` struct represents the recursive backtracking algorithm.
pub struct RecursiveBacktracker {}

impl RecursiveBacktracker {
    pub fn new() -> Self {
        Self {}
    }
}

impl On for RecursiveBacktracker {
    /// Executes the recursive backtracking algorithm on the given grid.
    ///
    /// # Arguments
    ///
    /// * `grid` - The grid on which to execute the algorithm.
    fn on(&self, grid: &mut dyn BaseGrid) {
        let cell = random_cell(grid);

        let mut stack: Vec<GridCell> = vec![];
        stack.push(cell);

        while !stack.is_empty() {
            let current = stack.last().unwrap();

            let neighbors = current
                .borrow()
                .neighbors()
                .into_iter()
                .filter(|(row, column)| {
                    let neighbor = grid.cell(*row, *column).unwrap();
                    neighbor.borrow().links().is_empty()
                })
                .collect::<Vec<(i32, i32)>>();

            if neighbors.is_empty() {
                stack.pop();
            } else {
                let neighbor = random_neighbor(grid, &neighbors);
                current.borrow_mut().link(neighbor.clone());
                stack.push(neighbor);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grids::base_grid::WithRowsAndColumns;

    use super::*;

    #[test]
    fn test_recursive_backtracker() {
        let mut grid = crate::grids::grid::Grid::new(5, 5);
        let recursive_backtracker = RecursiveBacktracker::new();

        recursive_backtracker.on(&mut grid);

        // Assert that all cells are linked
        for cell in grid.iter() {
            assert!(!cell.borrow().links().is_empty());
        }
    }
}
