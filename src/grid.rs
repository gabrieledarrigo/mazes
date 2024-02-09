use crate::cell::Cell;

pub struct Grid {
    rows: i32,
    columns: i32,
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(rows: i32, columns: i32) -> Self {
        let grid = Self::prepare_grid(rows, columns);

        Self {
            rows,
            columns,
            grid,
        }
    }

    fn prepare_grid(rows: i32, columns: i32) -> Vec<Vec<Cell>> {
        let mut grid = vec![vec![Cell::new(0, 0); columns as usize]; rows as usize];

        for row in 0..rows {
            for column in 0..columns {
                let mut cell = Cell::new(row, column);

                if row > 0 {
                    cell.set_north(Some((row - 1, column)));
                }

                if row < rows - 1 {
                    cell.set_south(Some((row + 1, column)));
                }

                if column > 0 {
                    cell.set_west(Some((row, column - 1)));
                }

                if column < columns - 1 {
                    cell.set_east(Some((row, column + 1)));
                }

                grid[row as usize][column as usize] = cell;
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_grid() {
        let rows = 3;
        let columns = 3;
        let grid = Grid::new(rows, columns);

        assert_eq!(grid.rows, rows);
        assert_eq!(grid.columns, columns);

        for row in 0..rows {
            for column in 0..columns {
                let cell = &grid.grid[row as usize][column as usize];

                if row > 0 {
                    assert_eq!(cell.north(), Some((row - 1, column)));
                }

                if row < rows - 1 {
                    assert_eq!(cell.south(), Some((row + 1, column)));
                }

                if column > 0 {
                    assert_eq!(cell.west(), Some((row, column - 1)));
                }

                if column < columns - 1 {
                    assert_eq!(cell.east(), Some((row, column + 1)));
                }
            }
        }
    }
}
