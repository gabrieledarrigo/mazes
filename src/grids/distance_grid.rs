use super::{
    base_grid::{GridCell, GridIterator, WithDisplay, WithRowsAndColumns},
    distances::Distances,
    grid::Grid,
    grid_display::GridDisplay,
};
use colored::Colorize;
use radix_fmt::radix_36;

/// Represents a grid with distances between cells.
pub struct DistanceGrid {
    grid: Grid,
    distances: Distances,
}

impl DistanceGrid {
    /// Creates a new `DistanceGrid` with the specified number of rows and columns.
    ///
    /// # Arguments
    ///
    /// * `rows` - The number of rows in the grid.
    /// * `columns` - The number of columns in the grid.
    ///
    /// # Returns
    ///
    /// A new `DistanceGrid` instance.
    pub fn new(rows: i32, columns: i32) -> Self {
        Self {
            distances: Distances::new((0, 0)),
            grid: Grid::new(rows, columns),
        }
    }

    /// Displays the grid with the path to the specified goal cell.
    ///
    /// # Arguments
    ///
    /// * `goal` - The goal cell to find the path to.
    ///
    /// # Returns
    ///
    /// A `GridDisplay` instance that can be used to display the grid.
    pub fn display_path_to(&mut self, goal: GridCell) -> GridDisplay {
        let root = self.cell(0, 0).unwrap().to_owned();
        self.distances
            .calculate(root, &self.grid)
            .path_to(goal, &self.grid);

        GridDisplay::new(
            &self.grid,
            Box::new(|cell: GridCell| {
                let row = cell.borrow_mut().row();
                let column = cell.borrow_mut().column();
                let distance = self.distances.get((row, column)).unwrap_or(&0);

                format!(
                    " {} ",
                    if *distance > 0 {
                        format!("{distance:X}")
                    } else {
                        " ".to_string()
                    }
                )
            }),
        )
    }
}

impl WithRowsAndColumns for DistanceGrid {
    fn rows(&self) -> i32 {
        self.grid.rows()
    }

    fn columns(&self) -> i32 {
        self.grid.columns()
    }

    fn cell(&self, row: i32, column: i32) -> Option<&GridCell> {
        self.grid.cell(row, column)
    }

    fn iter(&self) -> GridIterator {
        self.grid.iter()
    }

    fn each_row(&self) -> std::slice::Iter<'_, Vec<GridCell>> {
        self.grid.each_row()
    }
}

impl WithDisplay for DistanceGrid {
    /// Displays the grid with the distances between cells.
    ///
    /// # Returns
    ///
    /// A `GridDisplay` instance that can be used to display the grid.
    fn display(&mut self) -> GridDisplay {
        let root = self.cell(0, 0).unwrap().to_owned();
        self.distances.calculate(root, &self.grid);

        GridDisplay::new(
            &self.grid,
            Box::new(|cell: GridCell| {
                let row = cell.borrow_mut().row();
                let column = cell.borrow_mut().column();
                let distance = self.distances.get((row, column)).unwrap_or(&0);

                format!(" {:#} ", radix_36(*distance))
            }),
        )
    }

    /// Displays the grid with colors based on the distances between cells.
    ///
    /// # Returns
    ///
    /// A `GridDisplay` instance that can be used to display the grid.
    fn display_with_color(&mut self) -> GridDisplay {
        let root = self.cell(0, 0).unwrap().to_owned();
        self.distances.calculate(root, &self.grid);

        GridDisplay::new(
            &self.grid,
            Box::new(|cell: GridCell| {
                let row = cell.borrow_mut().row();
                let column = cell.borrow_mut().column();
                let distance = self.distances.get((row, column)).unwrap_or(&0);
                let max_distance = self.distances.max_distance().value();

                let intensity = f64::from(max_distance - distance) / f64::from(max_distance);
                let dark = (255.0 * intensity).floor();
                let bright = 128.0 + (127.0 * intensity).floor();

                format!(" {:#} ", radix_36(*distance))
                    .as_str()
                    .on_truecolor(dark as u8, dark as u8, bright as u8)
                    .to_string()
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_distance_grid() {
        let rows = 3;
        let columns = 3;
        let distance_grid = DistanceGrid::new(rows, columns);

        assert_eq!(distance_grid.grid, Grid::new(3, 3));
        assert_eq!(distance_grid.distances.get((0, 0)), Some(&0));
    }

    #[test]
    fn test_display_distance_grid() {
        let rows = 3;
        let columns = 3;
        let mut distance_grid = DistanceGrid::new(rows, columns);
        let display = distance_grid.display();

        assert_eq!(
            display.to_string().replace([' ', '\n'], "").trim(),
            "+---+---+---+
             | 0 | 0 | 0 |
             +---+---+---+
             | 0 | 0 | 0 |
             +---+---+---+
             | 0 | 0 | 0 |
             +---+---+---+"
                .replace([' ', '\n'], "")
                .trim(),
        );
    }
}
