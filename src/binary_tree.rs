use rand::Rng;

use crate::grid::Grid;

/// Implements the binary tree algorithm for generating mazes.
///
/// The binary tree algorithm works by iterating over each cell in the grid and randomly
/// linking it to either its north or east neighbor. This creates a maze with a bias towards
/// paths that go either north or east.
///
/// # Examples
///
/// ```
/// use mazes::grid::Grid;
/// use mazes::binary_tree::BinaryTree;
///
/// let mut grid = Grid::new(5, 5);
/// BinaryTree::on(&mut grid);
/// ```
///
/// In this example, a 5x5 grid is created and the binary tree algorithm is applied to it,
/// generating a maze with a bias towards paths that go either north or east.
pub struct BinaryTree {}

impl BinaryTree {
    /// Applies the binary tree algorithm to the given grid.
    ///
    /// # Arguments
    ///
    /// * `grid` - A mutable reference to the grid on which to apply the algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// use mazes::grid::Grid;
    /// use mazes::binary_tree::BinaryTree;
    ///
    /// let mut grid = Grid::new(5, 5);
    /// BinaryTree::on(&mut grid);
    /// ```
    ///
    /// In this example, a 5x5 grid is created and the binary tree algorithm is applied to it,
    /// generating a maze with a bias towards paths that go either north or east.
    pub fn on(grid: &mut Grid) {
        let mut rng = rand::thread_rng();

        for cell in grid.iter() {
            let mut neighbors = vec![];
            let mut cell = cell.borrow_mut();

            if let Some(north) = cell.north() {
                neighbors.push(north);
            }

            if let Some(east) = cell.east() {
                neighbors.push(east);
            }

            if neighbors.is_empty() {
                continue;
            }

            let index: usize = rng.gen_range(0..neighbors.len());
            let (neighbor_row, neighbor_column) = neighbors[index];
            let neighbor = grid.cell(neighbor_row, neighbor_column).unwrap().to_owned();

            cell.link(neighbor);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    use super::*;

    #[test]
    fn test_binary_tree_on() {
        let mut grid = Grid::new(3, 3);

        BinaryTree::on(&mut grid);

        // Verify that each cell is linked to either its north or east neighbor
        for cell in grid.iter() {
            let cell = cell.borrow();
            let links = cell.links();

            if cell.north().is_some() {
                assert!(
                    links.contains_key(&(cell.row() - 1, cell.column()))
                        || links.contains_key(&(cell.row(), cell.column() + 1))
                );
            }

            if cell.east().is_some() {
                assert!(
                    links.contains_key(&(cell.row() - 1, cell.column()))
                        || links.contains_key(&(cell.row(), cell.column() + 1))
                );
            }
        }
    }
}
