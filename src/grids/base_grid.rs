use super::{cell::Cell, grid_display::GridDisplay};
use std::{cell::RefCell, iter::Flatten, rc::Rc, slice::Iter};

pub type GridCell = Rc<RefCell<Cell>>;

/// The `WithRowsAndColumns` trait represents a grid structure with rows and columns.
pub trait WithRowsAndColumns {
    /// Returns the number of rows in the grid.
    fn rows(&self) -> i32;

    /// Returns the number of columns in the grid.
    fn columns(&self) -> i32;

    /// Returns a reference to the cell at the specified row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the cell.
    /// * `column` - The column index of the cell.
    ///
    /// # Returns
    ///
    /// An optional reference to the cell at the specified row and column.
    fn cell(&self, row: i32, column: i32) -> Option<&GridCell>;

    /// Returns an iterator over the cells of the grid.
    fn iter(&self) -> GridIterator;

    /// Returns an iterator over each row of the grid.
    fn each_row(&self) -> Iter<'_, Vec<GridCell>>;
}

/// The `WithDisplay` trait represents a grid structure with display capabilities.
pub trait WithDisplay {
    /// Returns a display representation of the grid.
    ///
    /// # Returns
    ///
    /// A `GridDisplay` instance that can be used to display the grid.
    fn display(&mut self) -> GridDisplay;

    fn display_with_color(&mut self) -> GridDisplay;
}

/// The `BaseGrid` trait represents a grid structure with rows, columns, and display capabilities.
pub trait BaseGrid: WithRowsAndColumns + WithDisplay {}

/// Implement the `BaseGrid` trait for any type that implements `WithRowsAndColumns` and `WithDisplay`.
impl<T> BaseGrid for T where T: WithRowsAndColumns + WithDisplay {}

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
