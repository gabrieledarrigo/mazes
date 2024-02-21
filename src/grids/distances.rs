use std::collections::HashMap;

/// Represents a collection of distances from a root cell to other cells in a grid.
#[derive(Debug)]
pub struct Distances {
    root: (i32, i32),
    cells: HashMap<(i32, i32), i32>,
}

impl Distances {
    /// Creates a new instance of `Distances` with the specified root cell.
    ///
    /// # Arguments
    ///
    /// * `root` - The root cell from which distances are measured.
    ///
    /// # Returns
    ///
    /// A new instance of `Distances`.
    pub fn new(root: (i32, i32)) -> Self {
        Self {
            root,
            cells: HashMap::new(),
        }
    }

    /// Sets the distance from the root cell to the specified cell.
    ///
    /// # Arguments
    ///
    /// * `cell` - The cell for which to set the distance.
    /// * `distance` - The distance from the root cell to the specified cell.
    pub fn set(&mut self, cell: (i32, i32), distance: i32) {
        self.cells.insert(cell, distance);
    }

    /// Returns a vector of references to all cells in the distances collection.
    ///
    /// # Returns
    ///
    /// A vector of references to all cells in the distances collection.
    pub fn cells(&self) -> Vec<&(i32, i32)> {
        self.cells.keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Distances;

    #[test]
    fn test_new() {
        let root = (0, 0);
        let distances = Distances::new(root);

        assert_eq!(distances.root, root);
        assert_eq!(distances.cells, HashMap::new());
    }

    #[test]
    fn test_set() {
        let mut distances = Distances::new((0, 0));
        distances.set((0, 1), 1);

        assert_eq!(distances.cells.get(&(0, 1)).unwrap().to_owned(), 1);
    }

    #[test]
    fn test_cells() {
        let mut distances = Distances::new((0, 0));

        for i in 1..3 {
            distances.set((0, i), i);
        }

        assert_eq!(distances.cells(), vec![&(0, 1), &(0, 2)]);
    }
}
