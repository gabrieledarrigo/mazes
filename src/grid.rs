use rand::Rng;

use crate::cell::Cell;

/// Represents a grid of cells.
pub struct Grid {
    rows: i32,
    columns: i32,
    grid: Vec<Vec<Cell>>,
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
        let grid = Self::prepare_grid(rows, columns);

        Self {
            rows,
            columns,
            grid,
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
        let mut grid = vec![vec![Cell::new(0, 0); columns as usize]; rows as usize];

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

                grid[row as usize][column as usize] = cell;
            }
        }

        grid
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
    fn cell(&self, row: i32, column: i32) -> Option<&Cell> {
        if (row >= 0 && row < self.rows) == false || (column >= 0 && column < self.columns) == false
        {
            return None;
        }

        Some(&self.grid[row as usize][column as usize])
    }

    /// Return a random cell from the grid.
    ///
    /// # Returns
    ///
    /// An optional reference to a random cell from the grid.
    fn random_cell(&self) -> Option<&Cell> {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..self.rows);
        let column = rng.gen_range(0..self.columns);

        self.cell(row, column)
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
                let cell = &grid.grid[row as usize][column as usize];

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
}
