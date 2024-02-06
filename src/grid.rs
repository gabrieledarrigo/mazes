use crate::cell::Cell;

pub struct Grid<'a> {
    rows: i32,
    columns: i32,
    grid: Vec<Vec<Cell<'a>>>,
}

impl<'a> Grid<'a> {
    pub fn new(rows: i32, columns: i32) -> Self {
        Self {
            rows,
            columns,
            grid: Self::prepare_grid(rows, columns),
        }
    }

    fn prepare_grid(rows: i32, columns: i32) -> Vec<Vec<Cell<'a>>> {
        let mut grid = vec![vec![Cell::new(0, 0); columns as usize]; rows as usize];

        for row in 0..rows {
            for column in 0..columns {
                grid[row as usize][column as usize] = Cell::new(row, column);
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
        let columns = 4;
        let grid = Grid::new(rows, columns);

        assert_eq!(grid.rows, rows);
        assert_eq!(grid.columns, columns);

        for row in 0..rows {
            for column in 0..columns {
                assert_eq!(
                    grid.grid[row as usize][column as usize],
                    Cell::new(row, column)
                );
            }
        }
    }
}
