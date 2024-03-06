use super::base_grid::{BaseGrid, GridCell};
use std::collections::HashMap;

/// Represents a collection of distances from a root cell to other cells in a grid.
#[derive(Debug)]
pub struct Distances {
    root: (i32, i32),
    cells: HashMap<(i32, i32), i32>,
}

impl Distances {
    /// Creates a new instance of `Distances` with the specified root cell.
    ///
    /// # Arguments
    ///
    /// * `root` - The root cell from which distances are measured.
    ///
    /// # Returns
    ///
    /// A new instance of `Distances`.
    pub fn new(root: (i32, i32)) -> Self {
        let mut cells = HashMap::new();
        cells.insert(root, 0);

        Self { root, cells }
    }

    /// Returns the distance from the root cell to the specified cell, if it exists.
    ///
    /// # Arguments
    ///
    /// * `cell` - The cell for which to retrieve the distance.
    ///
    /// # Returns
    ///
    /// The distance from the root cell to the specified cell, if it exists.
    pub fn get(&self, cell: (i32, i32)) -> Option<&i32> {
        self.cells.get(&cell)
    }

    /// Sets the distance from the root cell to the specified cell.
    ///
    /// # Arguments
    ///
    /// * `cell` - The cell for which to set the distance.
    /// * `distance` - The distance from the root cell to the specified cell.
    pub fn set(&mut self, cell: (i32, i32), distance: i32) {
        self.cells.insert(cell, distance);
    }

    /// Returns a vector of references to all cells in the distances collection.
    ///
    /// # Returns
    ///
    /// A vector of references to all cells in the distances collection.
    pub fn cells(&self) -> Vec<&(i32, i32)> {
        self.cells.keys().collect()
    }

    pub fn calculate(&mut self, root: GridCell, grid: &impl BaseGrid) -> &mut Self {
        let mut frontier = vec![root];

        while !frontier.is_empty() {
            let mut new_frontier = vec![];

            for cell in frontier {
                let cell = cell.borrow();

                for linked in cell.links().keys() {
                    if self.cells.contains_key(linked) {
                        continue;
                    }

                    let cell = (cell.row(), cell.column());

                    if let Some(distance) = self.get(cell) {
                        self.set(*linked, *distance + 1);

                        let new_cell = grid.cell(linked.0, linked.1).unwrap();
                        new_frontier.push(new_cell.to_owned())
                    }
                }
            }

            frontier = new_frontier;
        }

        self
    }

    pub fn path_to(&mut self, goal: GridCell, grid: &impl BaseGrid) -> Option<Self> {
        let mut current: std::rc::Rc<std::cell::RefCell<super::cell::Cell>> = goal;
        let mut breadcrumbs = Distances::new(self.root);

        let mut current_cell = current.borrow().to_row_and_column();

        if let Some(distance) = self.get(current_cell) {
            breadcrumbs.set(current_cell, *distance);
        } else {
            return None;
        }

        while current_cell != self.root {
            if let Some(distance) = self.get(current_cell) {
                let mut next_current = None;

                for (neighbour, _) in current.borrow().links() {
                    if let Some(neighbour_distance) = self.get(*neighbour) {
                        if *neighbour_distance < *distance {
                            breadcrumbs.set(*neighbour, *neighbour_distance);
                            next_current =
                                Some(grid.cell(neighbour.0, neighbour.1).unwrap().clone());
                            break;
                        }
                    } else {
                        return None;
                    }
                }

                if let Some(new_current) = next_current {
                    current = new_current;
                    current_cell = current.borrow().to_row_and_column();
                } else {
                    break;
                }
            } else {
                return None;
            }
        }

        Some(breadcrumbs)
    }
}

#[cfg(test)]
mod tests {
    use super::super::grid::Grid;
    use super::Distances;
    use crate::grids::base_grid::BaseGrid;
    use crate::BinaryTree;
    use std::collections::HashMap;

    #[test]
    fn test_new() {
        let root = (0, 0);
        let distances = Distances::new(root);
        let mut cells = HashMap::new();
        cells.insert(root, 0);

        assert_eq!(distances.root, root);
        assert_eq!(distances.cells, cells);
    }

    #[test]
    fn test_set() {
        let mut distances = Distances::new((0, 0));
        distances.set((0, 1), 1);

        assert_eq!(distances.cells.get(&(0, 1)).unwrap().to_owned(), 1);
    }

    #[test]
    fn test_cells() {
        let mut distances = Distances::new((0, 0));
        distances.set((0, 1), 1);
        distances.set((0, 2), 2);

        assert!(distances.cells().contains(&&(0, 0)));
        assert!(distances.cells().contains(&&(0, 1)));
        assert!(distances.cells().contains(&&(0, 2)));
    }

    #[test]
    fn test_calculate() {
        let mut grid = Grid::new(3, 3);
        BinaryTree::on(&mut grid);

        let root = grid.cell(0, 0).unwrap();
        let mut distances = Distances::new((0, 0));
        distances.calculate(root.to_owned(), &grid);

        // Verify the distances for specific cells
        assert_eq!(*distances.get((0, 0)).unwrap(), 0);
        assert_eq!(*distances.get((0, 1)).unwrap(), 1);
        assert_eq!(*distances.get((0, 2)).unwrap(), 2);

        // Verify the total number of cells in the distances collection
        assert_eq!(distances.cells().len(), 9);
    }

    #[test]
    fn test_path_to() {
        let mut grid = Grid::new(3, 3);
        BinaryTree::on(&mut grid);

        let root = grid.cell(0, 0).unwrap().clone();
        let goal = grid.cell(2, 2).unwrap().clone();

        let mut distances = Distances::new(root.borrow().to_row_and_column());
        distances.calculate(root, &grid);

        let path = distances.path_to(goal, &grid);

        assert_eq!(path.is_some(), true);
        let breadcrumbs = path.unwrap();
        assert_eq!(breadcrumbs.get((0, 0)), Some(&0));
    }
}
