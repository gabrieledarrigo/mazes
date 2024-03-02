use std::collections::HashMap;

use super::base_grid::GridCell;

/// Represents a cell in a maze grid.
#[derive(PartialEq, Debug, Clone)]
pub struct Cell {
    row: i32,
    column: i32,
    north: Option<(i32, i32)>,
    south: Option<(i32, i32)>,
    west: Option<(i32, i32)>,
    east: Option<(i32, i32)>,
    links: HashMap<(i32, i32), bool>,
}

impl Cell {
    /// Creates a new cell with the specified row and column.
    ///
    /// # Arguments
    ///
    /// * `row` - The row position in the grid
    /// * `column` - The column position in the grid
    pub fn new(row: i32, column: i32) -> Self {
        let links = HashMap::new();

        Self {
            north: None,
            south: None,
            west: None,
            east: None,
            row,
            column,
            links,
        }
    }

    /// Returns the row of the cell.
    ///
    /// # Returns
    ///
    /// The row of the cell.
    pub fn row(&self) -> i32 {
        self.row
    }

    /// Returns the column of the cell.
    ///
    /// # Returns
    ///
    /// The column of the cell.
    pub fn column(&self) -> i32 {
        self.column
    }

    /// Returns a reference to the links of the cell.
    pub fn links(&self) -> &HashMap<(i32, i32), bool> {
        &self.links
    }

    /// Links the cell to another cell.
    ///
    /// # Arguments
    ///
    /// * `other` - The coordinates of the cell to link to.
    pub fn link(&mut self, other: GridCell) {
        let (row, column) = (other.borrow().row, other.borrow().column);

        self.links.insert((row, column), true);

        other
            .borrow_mut()
            .links
            .insert((self.row, self.column), true);
    }

    /// Unlinks the cell from another cell.
    ///
    /// # Arguments
    ///
    /// * `other` - The coordinates of the cell to unlink from.
    pub fn unlink(&mut self, other: GridCell) {
        let (row, column) = (other.borrow().row, other.borrow().column);

        self.links.remove(&(row, column));
    }

    /// Returns the neighboring cells of the cell.
    ///
    /// # Returns
    ///
    /// A vector of neighboring cells.
    pub fn neighbors(&self) -> Vec<(i32, i32)> {
        let mut list = vec![];

        if let Some(cell) = self.north {
            list.push(cell);
        }

        if let Some(cell) = self.south {
            list.push(cell);
        }

        if let Some(cell) = self.west {
            list.push(cell);
        }

        if let Some(cell) = self.east {
            list.push(cell);
        }

        list
    }

    /// Returns the north neighbor of the cell.
    pub fn north(&self) -> Option<(i32, i32)> {
        self.north
    }

    /// Sets the north neighbor of the cell.
    ///
    /// # Arguments
    ///
    /// * `north` - The north neighbor of the cell.
    pub fn set_north(&mut self, north: Option<(i32, i32)>) {
        self.north = north;
    }

    /// Returns the south neighbor of the cell.
    ///
    /// # Returns
    ///
    /// The south neighbor of the cell.
    pub fn south(&self) -> Option<(i32, i32)> {
        self.south
    }

    /// Sets the south neighbor of the cell.
    ///
    /// # Arguments
    ///
    /// * `south` - The south neighbor of the cell.
    pub fn set_south(&mut self, south: Option<(i32, i32)>) {
        self.south = south;
    }

    /// Returns the west neighbor of the cell.
    ///
    /// # Returns
    ///
    /// The west neighbor of the cell.
    pub fn west(&self) -> Option<(i32, i32)> {
        self.west
    }

    /// Sets the west neighbor of the cell.
    ///
    /// # Arguments
    ///
    /// * `west` - The west neighbor of the cell.
    pub fn set_west(&mut self, west: Option<(i32, i32)>) {
        self.west = west;
    }

    /// Returns the east neighbor of the cell.
    pub fn east(&self) -> Option<(i32, i32)> {
        self.east
    }

    /// Sets the east neighbor of the cell.
    ///
    /// # Arguments
    ///
    /// * `east` - The east neighbor of the cell.
    pub fn set_east(&mut self, east: Option<(i32, i32)>) {
        self.east = east;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grids::grid::Grid;

    #[test]
    fn test_new_cell() {
        let cell = Cell::new(0, 0);

        assert_eq!(cell.row, 0);
        assert_eq!(cell.column, 0);
        assert!(cell.links.is_empty());
    }

    #[test]
    fn test_link_cells() {
        let mut cell1: Cell = Cell::new(0, 0);
        let cell2 = Grid::new_grid_cell(1, 1);

        cell1.link(cell2);

        assert_eq!(cell1.links.get(&(1, 1)), Some(&true));
    }

    #[test]
    fn test_unlink() {
        let mut cell1 = Cell::new(0, 0);
        let cell2 = Grid::new_grid_cell(1, 1);

        cell1.link(cell2.clone());
        cell1.unlink(cell2);

        assert_eq!(cell1.links.get(&(1, 1)), None);
    }

    #[test]
    fn test_links() {
        let mut cell1 = Cell::new(0, 0);
        let cell2 = Grid::new_grid_cell(1, 1);

        cell1.link(cell2);

        assert_eq!(cell1.links(), &cell1.links);
    }

    #[test]
    fn test_neighbors() {
        let mut cell = Cell::new(1, 1);
        let north = (0, 1);
        let south = (2, 1);
        let west = (1, 0);
        let east = (2, 2);

        cell.set_north(Some(north));
        cell.set_south(Some(south));
        cell.set_west(Some(west));
        cell.set_east(Some(east));

        assert_eq!(cell.neighbors(), vec![north, south, west, east]);
    }
}
