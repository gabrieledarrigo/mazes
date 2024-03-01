use super::cell::Cell;
use rand::Rng;
use std::{cell::RefCell, fmt::Display, iter::Flatten, rc::Rc, slice::Iter};

pub type GridCell = Rc<RefCell<Cell>>;

pub struct GridDisplay<'a> {
    grid: &'a Grid,
    cell_content: String,
}

impl<'a> Display for GridDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self.grid;

        let mut output = String::from("+");
        output.push_str(&"---+".repeat(grid.columns as usize));
        output.push_str(&"\n");

        for row in 0..grid.rows {
            let mut top = String::from("|");
            let mut bottom = String::from("+");

            for column in 0..grid.columns {
                let body = self.cell_content.to_owned();
                let mut east_boundary = String::from("|");
                let mut south_boundary = String::from("---");
                let corner = String::from("+");

                if let Some(cell) = grid.cell(row, column) {
                    let cell = cell.borrow();
                    let east = cell.east().unwrap_or((-1, -1));
                    let south = cell.south().unwrap_or((-1, -1));

                    if cell.links().contains_key(&east) {
                        east_boundary = String::from(" ");
                    }

                    if cell.links().contains_key(&south) {
                        south_boundary = String::from("   ");
                    }
                }

                top.push_str(&body);
                top.push_str(&east_boundary);

                bottom.push_str(&south_boundary);
                bottom.push_str(&corner);
            }

            top.push('\n');
            bottom.push('\n');

            output.push_str(&top);
            output.push_str(&bottom);
        }

        write!(f, "{}", output)
    }
}

/// Represents a grid of cells.
#[derive(Debug, Clone)]
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
    pub fn cell(&self, row: i32, column: i32) -> Option<&GridCell> {
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
    pub fn random_cell(&self) -> Option<&GridCell> {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0..self.rows);
        let column = rng.gen_range(0..self.columns);

        self.cell(row, column)
    }

    /// Return a struct to iterate the cells mutably.
    ///
    /// # Returns
    ///
    /// A GridMutIterator struct
    pub fn iter(&self) -> GridIterator {
        GridIterator::new(self.cells.iter().flatten())
    }

    /// Return an iterator over the rows of the grid.
    ///
    /// # Returns
    ///
    /// An iterator over the rows of the grid.
    pub fn each_row(&self) -> Iter<'_, Vec<GridCell>> {
        self.cells.iter()
    }

    pub fn display(&self, cell_content: String) -> GridDisplay<'_> {
        GridDisplay {
            grid: self,
            cell_content,
        }
    }
}

/// An iterator over the cells of a grid.
pub struct GridIterator<'a> {
    cells: Flatten<Iter<'a, Vec<GridCell>>>,
}

impl<'a> GridIterator<'a> {
    /// Creates a new grid iterator.
    ///
    /// # Arguments
    ///
    /// * `cells` - An iterator over the cells of the grid.
    pub fn new(cells: Flatten<Iter<'a, Vec<GridCell>>>) -> Self {
        Self { cells }
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = &'a GridCell;

    /// Returns the next cell in the grid.
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
