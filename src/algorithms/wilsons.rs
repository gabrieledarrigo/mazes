use super::On;
use crate::{
    grids::base_grid::BaseGrid,
    utils::random::{random_cell, random_neighbor},
};
use rand::Rng;

/// The Wilsons struct represents the Wilson's algorithm for generating mazes.
pub struct Wilsons {}

impl Wilsons {
    pub fn new() -> Self {
        Self {}
    }
}

impl On for Wilsons {
    /// Generates a maze using the Wilson's algorithm on the given grid.
    ///
    /// # Arguments
    ///
    /// * `grid` - The grid on which to generate the maze.
    fn on(&self, grid: &mut dyn BaseGrid) {
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
                cell = random_neighbor(grid, &cell.clone().borrow().neighbors());

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
    use crate::grids::{base_grid::WithRowsAndColumns, grid::Grid};

    #[test]
    fn test_wilsons_on() {
        let mut grid = Grid::new(3, 3);
        let wilsons = Wilsons::new();

        wilsons.on(&mut grid);

        // Assert that all cells are linked
        for cell in grid.iter() {
            assert!(!cell.borrow().links().is_empty());
        }
    }
}
