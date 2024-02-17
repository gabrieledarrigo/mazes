use crate::grid::Grid;
use rand::Rng;

#[derive(Debug)]
pub struct BinaryTree {}

impl BinaryTree {
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

            if let Some(south) = cell.south() {
                neighbors.push(south);
            }

            if let Some(west) = cell.west() {
                neighbors.push(west);
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
    use super::*;

    #[test]
    fn test_binary_tree_on() {
        let mut grid = Grid::new(3, 3);
        BinaryTree::on(&mut grid);

        // Verify that each cell is linked to one of its neighbors
        for cell in grid.iter() {
            let cell = cell.borrow();
            let neighbors = cell.neighbors();

            assert!(!neighbors.is_empty());
        }
    }
}
