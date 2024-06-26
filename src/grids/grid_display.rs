use super::{base_grid::GridCell, grid::Grid};
use crate::grids::base_grid::WithRowsAndColumns;
use std::fmt::Display;

/// A struct that holds a Grid and a function to format the content of a Cell.
pub struct GridDisplay<'a> {
    grid: &'a Grid,
    cell_content: Box<dyn Fn(GridCell) -> String + 'a>,
}

impl<'a> GridDisplay<'a> {
    /// Creates a new `GridDisplay` instance.
    ///
    /// # Arguments
    ///
    /// * `grid` - A reference to the grid.
    /// * `cell_content` - A closure that takes a `GridCell` and returns a `String` representing the content of the cell.
    ///
    /// # Returns
    ///
    /// A new `GridDisplay` instance.
    pub fn new(grid: &'a Grid, cell_content: Box<dyn Fn(GridCell) -> String + 'a>) -> Self {
        Self { grid, cell_content }
    }
}

impl<'a> Display for GridDisplay<'a> {
    /// Formats the `GridDisplay` instance for display.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the output to.
    ///
    /// # Returns
    ///
    /// A `std::fmt::Result` indicating the success or failure of the formatting operation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self.grid as &dyn WithRowsAndColumns;

        let mut output = String::from("+");
        output.push_str(&"---+".repeat(grid.columns() as usize));
        output.push('\n');

        for row in 0..grid.rows() {
            let mut top = String::from("|");
            let mut bottom = String::from("+");

            for column in 0..grid.columns() {
                let body = (self.cell_content)(self.grid.cell(row, column).unwrap().clone());
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

        write!(f, "{output}")
    }
}
