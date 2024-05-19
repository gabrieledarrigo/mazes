use super::base_grid::{GridCell, WithRowsAndColumns};
use std::collections::HashMap;

/// Represents the maximum distance in a Grid from a root cell.
pub struct MaxDistance {
    value: i32,
}

impl MaxDistance {
    /// Creates a new instance of `MaxDistance` with the specified maximum cell and distance.
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum distance from the root cell to the maximum cell.
    ///
    /// # Returns
    ///
    /// A new instance of `MaxDistance`.
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    /// Returns the maximum distance from the root cell to the maximum cell.
    ///
    /// # Returns
    ///
    /// The maximum distance from the root cell to the maximum cell.
    pub fn value(&self) -> i32 {
        self.value
    }
}

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

    /// Calculates the distances from the root cell to all other cells in the grid.
    ///
    /// # Arguments
    ///
    /// * `root` - The root cell from which to calculate the distances.
    /// * `grid` - The grid on which the distances are calculated.
    ///
    /// # Returns
    ///
    /// A mutable reference to `Self` (the `Distances` instance) after the calculation is complete.
    pub fn calculate(&mut self, root: GridCell, grid: &impl WithRowsAndColumns) -> &mut Self {
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
                        new_frontier.push(new_cell.to_owned());
                    }
                }
            }

            frontier = new_frontier;
        }

        self
    }

    /// Calculates the shortest path from the root cell to the specified goal cell.
    ///
    /// # Arguments
    ///
    /// * `goal` - The goal cell to which the shortest path is calculated.
    /// * `grid` - The grid on which the shortest path is calculated.
    ///
    /// # Returns
    ///
    /// A mutable reference to `Self` (the `Distances` instance) after the shortest path calculation is complete.
    pub fn path_to(&mut self, goal: GridCell, grid: &impl WithRowsAndColumns) -> &mut Self {
        let mut current: std::rc::Rc<std::cell::RefCell<super::cell::Cell>> = goal;
        let mut breadcrumbs = Distances::new(self.root);

        let mut current_cell = current.borrow().to_row_and_column();

        if let Some(distance) = self.get(current_cell) {
            breadcrumbs.set(current_cell, *distance);
        } else {
            return self;
        }

        while current_cell != self.root {
            if let Some(distance) = self.get(current_cell) {
                let mut next_current = None;

                for neighbour in current.borrow().links().keys() {
                    if let Some(neighbour_distance) = self.get(*neighbour) {
                        if *neighbour_distance < *distance {
                            breadcrumbs.set(*neighbour, *neighbour_distance);
                            next_current =
                                Some(grid.cell(neighbour.0, neighbour.1).unwrap().clone());
                            break;
                        }
                    } else {
                        return self;
                    }
                }

                if let Some(new_current) = next_current {
                    current = new_current;
                    current_cell = current.borrow().to_row_and_column();
                } else {
                    break;
                }
            } else {
                return self;
            }
        }

        self.cells = breadcrumbs.cells;

        self
    }

    /// Calculates the maximum distance in the grid from the root cell to any other cell.
    ///
    /// # Returns
    ///
    /// A `MaxDistance` struct containing the maximum cell and distance.
    pub fn max_distance(&self) -> MaxDistance {
        let mut max_distance = 0;

        for (_, distance) in self.cells.clone() {
            if distance > max_distance {
                max_distance = distance;
            }
        }

        MaxDistance::new(max_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::super::grid::Grid;
    use super::Distances;
    use crate::algorithms::On;
    use crate::grids::base_grid::WithRowsAndColumns;
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
    fn test_calculate() {
        let mut grid = Grid::new(3, 3);
        let binary_tree = BinaryTree::new();
        binary_tree.on(&mut grid);

        let root = grid.cell(0, 0).unwrap();
        let mut distances = Distances::new((0, 0));
        distances.calculate(root.to_owned(), &grid);

        // Verify the distances for specific cells
        assert_eq!(*distances.get((0, 0)).unwrap(), 0);
        assert_eq!(*distances.get((0, 1)).unwrap(), 1);
        assert_eq!(*distances.get((0, 2)).unwrap(), 2);

        // Verify the total number of cells in the distances collection
        assert_eq!(distances.cells.keys().len(), 9);
    }

    #[test]
    fn test_path_to() {
        let mut grid = Grid::new(3, 3);
        let binary_tree = BinaryTree::new();
        binary_tree.on(&mut grid);

        let root = grid.cell(0, 0).unwrap().clone();
        let goal = grid.cell(2, 2).unwrap().clone();

        let mut distances = Distances::new(root.borrow().to_row_and_column());
        distances.calculate(root, &grid).path_to(goal, &grid);

        assert_eq!(distances.get((0, 0)), Some(&0));
    }

    #[test]
    fn test_max() {
        let root = (0, 0);
        let mut distances = Distances::new(root);
        distances.set((1, 0), 1);
        distances.set((2, 0), 2);
        distances.set((3, 0), 3);

        let max_distance = distances.max_distance();

        assert_eq!(max_distance.value, 3);
    }
}
