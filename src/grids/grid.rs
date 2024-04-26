use super::{
    base_grid::{GridCell, GridIterator, WithDisplay, WithRowsAndColumns},
    cell::Cell,
    grid_display::GridDisplay,
};
use std::{cell::RefCell, rc::Rc, slice::Iter};

/// Represents a grid of cells.
#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    rows: i32,
    columns: i32,
    cells: Vec<Vec<GridCell>>,
}

impl Grid {
    /// Creates a new grid with the specified number of rows and columns.
    ///
    /// # Arguments
    ///
    /// * `rows` - The number of rows in the grid.
    /// * `columns` - The number of columns in the grid.
    ///
    /// # Returns
    ///
    /// A new `Grid` instance.
    pub fn new(rows: i32, columns: i32) -> Self {
        let cells = Self::prepare_grid(rows, columns);

        Self {
            rows,
            columns,
            cells,
        }
    }

    /// Prepares the grid by initializing the cells and setting their neighbors.
    ///
    /// # Arguments
    ///
    /// * `rows` - The number of rows in the grid.
    /// * `columns` - The number of columns in the grid.
    ///
    /// # Returns
    ///
    /// A vector of vectors representing the grid.
    fn prepare_grid(rows: i32, columns: i32) -> Vec<Vec<GridCell>> {
        let mut cells = vec![vec![Self::new_grid_cell(0, 0); columns as usize]; rows as usize];

        for row in 0..rows {
            for column in 0..columns {
                cells[row as usize][column as usize] = Self::new_grid_cell(row, column)
            }
        }

        for row in 0..rows {
            for column in 0..columns {
                let cell = &mut cells[row as usize][column as usize].borrow_mut();

                if row > 0 {
                    cell.set_north(Some((row - 1, column)));
                }

                if row < rows - 1 {
                    cell.set_south(Some((row + 1, column)));
                }

                if column > 0 {
                    cell.set_west(Some((row, column - 1)));
                }

                if column < columns - 1 {
                    cell.set_east(Some((row, column + 1)));
                }
            }
        }

        cells
    }

    /// Returns a new grid cell at the specified row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the cell.
    /// * `column` - The column index of the cell.
    ///
    /// # Returns
    ///
    /// A new grid cell.
    pub fn new_grid_cell(row: i32, column: i32) -> GridCell {
        Rc::new(RefCell::new(Cell::new(row, column)))
    }
}

impl WithRowsAndColumns for Grid {
    /// Returns the number of rows in the grid.
    fn rows(&self) -> i32 {
        self.rows
    }

    /// Returns the number of columns in the grid.
    fn columns(&self) -> i32 {
        self.columns
    }

    /// Return the cell at the specified row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the cell.
    /// * `column` - The column index of the cell.
    ///
    /// # Returns
    ///
    /// An optional reference to the cell at the specified position.
    fn cell(&self, row: i32, column: i32) -> Option<&GridCell> {
        if (row >= 0 && row < self.rows) == false || (column >= 0 && column < self.columns) == false
        {
            return None;
        }

        Some(&self.cells[row as usize][column as usize])
    }

    /// Return a struct to iterate the cells mutably.
    ///
    /// # Returns
    ///
    /// A GridMutIterator struct
    fn iter(&self) -> GridIterator {
        GridIterator::new(self.cells.iter().flatten())
    }

    /// Return an iterator over the rows of the grid.
    ///
    /// # Returns
    ///
    /// An iterator over the rows of the grid.
    fn each_row(&self) -> Iter<'_, Vec<GridCell>> {
        self.cells.iter()
    }
}

impl WithDisplay for Grid {
    /// Displays the grid.
    ///
    /// This method returns a `GridDisplay` instance that can be used to display the grid.
    ///
    /// # Returns
    ///
    /// A `GridDisplay` instance for displaying the grid.
    fn display(&mut self) -> GridDisplay {
        GridDisplay::new(self, Box::new(|_| String::from("   ")))
    }

    /// Displays the grid.
    ///
    /// On the Grid structure, display_with_color doesn't render any color, and simply displays the grid.
    /// It internally calls the `display` method to obtain the grid representation.
    ///
    /// # Returns
    ///
    /// A `GridDisplay` instance for displaying the grid.
    fn display_with_color(&mut self) -> GridDisplay {
        self.display()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_grid() {
        let rows = 3;
        let columns = 3;
        let grid = Grid::new(rows, columns);

        assert_eq!(grid.rows, rows);
        assert_eq!(grid.columns, columns);

        for row in 0..rows {
            for column in 0..columns {
                let cell = &grid.cells[row as usize][column as usize].borrow();

                if row > 0 {
                    assert_eq!(cell.north(), Some((row - 1, column)));
                }

                if row < rows - 1 {
                    assert_eq!(cell.south(), Some((row + 1, column)));
                }

                if column > 0 {
                    assert_eq!(cell.west(), Some((row, column - 1)));
                }

                if column < columns - 1 {
                    assert_eq!(cell.east(), Some((row, column + 1)));
                }
            }
        }
    }

    #[test]
    fn test_get_cell_valid() {
        let rows = 3;
        let columns = 3;
        let grid = Grid::new(rows, columns);

        for row in 0..rows {
            for column in 0..columns {
                let cell = grid.cell(row, column).unwrap().borrow();

                assert_eq!(cell.row(), row);
                assert_eq!(cell.column(), column);
            }
        }
    }

    #[test]
    fn test_get_cell_invalid() {
        let rows = 3;
        let columns = 3;
        let grid = Grid::new(rows, columns);

        let invalid_row = -1;
        let invalid_column = columns + 1;

        let invalid_cell = grid.cell(invalid_row, invalid_column);

        assert!(invalid_cell.is_none());
    }

    #[test]
    fn test_iter_mut() {
        let rows = 3;
        let columns = 3;
        let grid = Grid::new(rows, columns);

        let mut iter = grid.iter();

        for row in 0..rows {
            for column in 0..columns {
                let cell = iter.next().unwrap().borrow();

                assert_eq!(cell.row(), row);
                assert_eq!(cell.column(), column);
            }
        }

        assert!(iter.next().is_none());
    }
}
