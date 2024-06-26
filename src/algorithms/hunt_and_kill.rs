use super::On;
use crate::{
    grids::base_grid::BaseGrid,
    utils::random::{random_cell, random_neighbor},
};

/// The `HuntAndKill` struct represents the Hunt and Kill algorithm for generating mazes.
pub struct HuntAndKill {}

impl HuntAndKill {
    pub fn new() -> Self {
        Self {}
    }
}

impl On for HuntAndKill {
    /// Executes the Hunt and Kill algorithm on the given grid.
    ///
    /// # Arguments
    ///
    /// * `grid` - The grid on which to execute the algorithm.
    fn on(&self, grid: &mut dyn BaseGrid) {
        let mut current = Some(random_cell(grid));

        while let Some(ref cell) = current {
            let neighbors = cell.borrow().neighbors();

            let unvisited = neighbors
                .iter()
                .filter(|(row, column)| {
                    let neighbor = grid.cell(*row, *column).unwrap();
                    neighbor.borrow().links().is_empty()
                })
                .map(|&(row, column)| (row, column))
                .collect::<Vec<(i32, i32)>>();

            if !unvisited.is_empty() {
                let neighbor = random_neighbor(grid, &unvisited);
                cell.borrow_mut().link(neighbor.clone());
                current = Some(neighbor);
            } else {
                current = None;

                for cell in grid.iter() {
                    let neighbors = cell.borrow().neighbors();

                    let visited = neighbors
                        .iter()
                        .filter(|(row, column)| {
                            let neighbor = grid.cell(*row, *column).unwrap();
                            !neighbor.borrow().links().is_empty()
                        })
                        .map(|&(row, column)| (row, column))
                        .collect::<Vec<(i32, i32)>>();

                    if cell.borrow().links().is_empty() && !visited.is_empty() {
                        current = Some(cell.clone());
                        let neighbor = random_neighbor(grid, &visited);
                        cell.borrow_mut().link(neighbor.clone());
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grids::{base_grid::WithRowsAndColumns, grid::Grid};

    #[test]
    fn test_hunt_and_kill() {
        let mut grid = Grid::new(5, 5);
        let hunt_and_kill = HuntAndKill::new();

        hunt_and_kill.on(&mut grid);

        // Assert that all cells are linked
        for cell in grid.iter() {
            assert!(!cell.borrow().links().is_empty());
        }
    }
}
