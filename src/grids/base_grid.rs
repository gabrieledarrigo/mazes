use super::cell::Cell;
use std::{cell::RefCell, iter::Flatten, rc::Rc, slice::Iter};

pub type GridCell = Rc<RefCell<Cell>>;

/// The `BaseGrid` trait represents a grid structure.
pub trait BaseGrid {
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
