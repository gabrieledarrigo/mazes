use std::{iter::Flatten, slice::Iter};

use rand::Rng;

use crate::cell::Cell;

/// Represents a grid of cells.
pub struct Grid {
    rows: i32,
    columns: i32,
    cells: Vec<Vec<Cell>>,
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
    fn prepare_grid(rows: i32, columns: i32) -> Vec<Vec<Cell>> {
        let mut cells = vec![vec![Cell::new(0, 0); columns as usize]; rows as usize];

        for row in 0..rows {
            for column in 0..columns {
                let mut cell = Cell::new(row, column);

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

                cells[row as usize][column as usize] = cell;
            }
        }

        cells
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
    pub fn cell(&self, row: i32, column: i32) -> Option<&Cell> {
        if (row >= 0 && row < self.rows) == false || (column >= 0 && column < self.columns) == false
        {
            return None;
        }

        Some(&self.cells[row as usize][column as usize])
    }

    /// Return a random cell from the grid.
    ///
    /// # Returns
    ///
    /// An optional reference to a random cell from the grid.
    pub fn random_cell(&self) -> Option<&Cell> {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..self.rows);
        let column = rng.gen_range(0..self.columns);

        self.cell(row, column)
    }

    /// Return a struct to iterate the cells.
    ///
    /// # Returns
    ///
    /// A GridIterator struct
    pub fn iter(&self) -> GridIterator {
        GridIterator::new(self.cells.iter().flatten())
    }
}

/// Represents an iterator over the cells in a grid.
pub struct GridIterator<'a> {
    cells: Flatten<Iter<'a, Vec<Cell>>>,
}

impl<'a> GridIterator<'a> {
    /// Creates a new `GridIterator` instance.
    ///
    /// # Arguments
    ///
    /// * `cells` - The iterator over the cells in the grid.
    ///
    /// # Returns
    ///
    /// A new `GridIterator` instance.
    pub fn new(cells: Flatten<Iter<'a, Vec<Cell>>>) -> Self {
        Self { cells }
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = &'a Cell;

    /// Advances the iterator and returns the next cell in the grid.
    ///
    /// # Returns
    ///
    /// An optional reference to the next cell in the grid.
    fn next(&mut self) -> Option<Self::Item> {
        self.cells.next()
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
                let cell = &grid.cells[row as usize][column as usize];

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
                let cell = grid.cell(row, column);

                assert!(cell.is_some());
                assert_eq!(cell.unwrap().row(), row);
                assert_eq!(cell.unwrap().column(), column);
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
    fn test_iter() {
        let rows = 3;
        let columns = 3;
        let grid = Grid::new(rows, columns);

        let mut iter = grid.iter();

        for row in 0..rows {
            for column in 0..columns {
                let cell = iter.next();

                assert!(cell.is_some());
                assert_eq!(cell.unwrap().row(), row);
                assert_eq!(cell.unwrap().column(), column);
            }
        }

        assert!(iter.next().is_none());
    }
}
