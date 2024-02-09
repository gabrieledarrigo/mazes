use std::collections::HashMap;

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
            east: None,
            west: None,
            row,
            column,
            links,
        }
    }

    /// Returns the row of the cell.
    pub fn row(&self) -> i32 {
        self.row
    }

    /// Returns the column of the cell.
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
    /// * `cell` - The cell to link to.
    /// * `bidirectional` - Whether the link should be bidirectional.
    pub fn link(&mut self, cell: &mut Cell, bidirectional: bool) {
        self.links.insert((cell.row, cell.column), true);

        if bidirectional {
            cell.link(self, false);
        }
    }

    /// Unlinks the cell from another cell.
    ///
    /// # Arguments
    ///
    /// * `cell` - The cell to unlink from.
    pub fn unlink(&mut self, cell: &mut Cell) {
        self.links.remove(&(cell.row, cell.column));

        if let Some(_) = cell.links.get(&(self.row, self.column)) {
            cell.links.remove(&(self.row, self.column));
        }
    }

    /// Returns the neighboring cells of the cell.
    pub fn neighbors(&self) -> Vec<(i32, i32)> {
        let mut list = vec![];

        if let Some(cell) = self.north {
            list.push(cell);
        }

        if let Some(cell) = self.south {
            list.push(cell);
        }

        if let Some(cell) = self.east {
            list.push(cell);
        }

        if let Some(cell) = self.west {
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

    /// Returns the west neighbor of the cell.
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut cell2 = Cell::new(1, 1);

        cell1.link(&mut cell2, true);

        assert_eq!(cell1.links.get(&(cell2.row, cell2.column)), Some(&true));
        assert_eq!(cell2.links.get(&(cell1.row, cell1.column)), Some(&true));
    }

    #[test]
    fn test_unlink() {
        let mut cell1 = Cell::new(0, 0);
        let mut cell2 = Cell::new(1, 1);

        let row = cell2.row;
        let column = cell2.column;
        let links = cell2.links().clone();

        cell1.link(&mut cell2, true);
        cell1.unlink(&mut cell2);

        assert_eq!(cell1.links().get(&(row, column)), None);
        assert_eq!(links.get(&(cell1.row, cell1.column)), None);
    }

    #[test]
    fn test_links() {
        let mut cell1 = Cell::new(0, 0);
        let mut cell2 = Cell::new(1, 1);

        cell1.link(&mut cell2, true);

        assert_eq!(cell1.links(), &cell1.links);
    }

    #[test]
    fn test_neighbors() {
        let mut cell = Cell::new(1, 1);
        let north = (0, 1);
        let south = (2, 1);
        let east = (2, 2);
        let west = (1, 0);

        cell.set_north(Some(north));
        cell.set_south(Some(south));
        cell.set_east(Some(east));
        cell.set_west(Some(west));

        assert_eq!(cell.neighbors(), vec![north, south, east, west]);
    }
}
