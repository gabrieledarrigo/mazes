use std::collections::HashMap;

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

    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn column(&self) -> i32 {
        self.column
    }

    pub fn links(&self) -> &HashMap<(i32, i32), bool> {
        &self.links
    }

    pub fn link(&mut self, cell: &mut Cell, bidirectional: bool) {
        self.links.insert((cell.row, cell.column), true);

        if bidirectional {
            cell.link(self, false);
        }
    }

    pub fn unlink(&mut self, cell: &mut Cell) {
        self.links.remove(&(cell.row, cell.column));

        if let Some(_) = cell.links.get(&(self.row, self.column)) {
            cell.links.remove(&(self.row, self.column));
        }
    }

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

    pub fn north(&self) -> Option<(i32, i32)> {
        self.north
    }

    pub fn set_north(&mut self, north: Option<(i32, i32)>) {
        self.north = north;
    }

    pub fn south(&self) -> Option<(i32, i32)> {
        self.south
    }

    pub fn set_south(&mut self, south: Option<(i32, i32)>) {
        self.south = south;
    }

    pub fn east(&self) -> Option<(i32, i32)> {
        self.east
    }

    pub fn set_east(&mut self, east: Option<(i32, i32)>) {
        self.east = east;
    }

    pub fn west(&self) -> Option<(i32, i32)> {
        self.west
    }

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
