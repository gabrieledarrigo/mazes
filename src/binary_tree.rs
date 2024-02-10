use rand::Rng;

use crate::grid::Grid;

#[derive(Debug)]
pub struct BinaryTree {}

impl BinaryTree {
    pub fn on(grid: &mut Grid) {
        let mut rng = rand::thread_rng();
        let temp_grid = grid.clone();

        for cell in grid.iter_mut() {
            let mut neighbors = vec![];

            if let Some(north) = cell.north() {
                neighbors.push(north);
            }

            if let Some(east) = cell.east() {
                neighbors.push(east);
            }

            if neighbors.is_empty() {
                continue;
            }

            let index = rng.gen_range(0..neighbors.len());
            let (row, column) = neighbors[index];

            if let Some(neighbor) = temp_grid.cell(row, column) {
                cell.link(&mut neighbor.clone(), false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_on() {
        let mut grid = Grid::new(5, 5);

        BinaryTree::on(&mut grid);
    }
}
