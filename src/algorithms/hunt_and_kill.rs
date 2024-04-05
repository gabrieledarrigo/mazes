use crate::{grids::base_grid::BaseGrid, utils::utils::*};

pub struct HuntAndKill {}

impl HuntAndKill {
    pub fn on(grid: &impl BaseGrid) {
        let mut current = Some(random_cell(grid));

        while let Some(ref cell) = current {
            let neighbors = cell.borrow().neighbors();

            let unvisited = neighbors
                .iter()
                .filter(|(row, column)| {
                    let neighbor = grid.cell(*row, *column).unwrap();
                    neighbor.borrow().links().is_empty()
                })
                .map(|&(row, column)| (row, column))
                .collect::<Vec<(i32, i32)>>();

            if !unvisited.is_empty() {
                let neighbor = random_neighbor(grid, unvisited);
                cell.borrow_mut().link(neighbor.clone());
                current = Some(neighbor);
            } else {
                current = None;

                for cell in grid.iter() {
                    let neighbors = cell.borrow().neighbors();

                    let visited = neighbors
                        .iter()
                        .filter(|(row, column)| {
                            let neighbor = grid.cell(*row, *column).unwrap();
                            neighbor.borrow().links().is_empty() == false
                        })
                        .map(|&(row, column)| (row, column))
                        .collect::<Vec<(i32, i32)>>();

                    if cell.borrow().links().is_empty() && !visited.is_empty() {
                        current = Some(cell.clone());
                        let neighbor = random_neighbor(grid, visited);
                        cell.borrow_mut().link(neighbor.clone());
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grids::grid::Grid;

    #[test]
    fn test_hunt_and_kill() {
        let mut grid = Grid::new(5, 5);

        HuntAndKill::on(&mut grid);

        // Assert that all cells are linked
        for cell in grid.iter() {
            assert!(!cell.borrow().links().is_empty());
        }
    }
}
