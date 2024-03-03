use super::{
    base_grid::{BaseGrid, GridCell, GridIterator},
    distances::Distances,
    grid::{Grid, GridDisplay},
};

pub struct DistanceGrid {
    grid: Grid,
    distances: Distances,
}

impl DistanceGrid {
    pub fn new(rows: i32, columns: i32) -> Self {
        Self {
            distances: Distances::new((0, 0)),
            grid: Grid::new(rows, columns),
        }
    }

    pub fn display(&mut self) -> GridDisplay<'_, impl Fn(GridCell) -> String + '_> {
        let root = self.cell(0, 0).unwrap().to_owned();
        self.distances.calculate(root, &self.grid);

        GridDisplay::new(&self.grid, |cell: GridCell| {
            let row = cell.borrow_mut().row();
            let column = cell.borrow_mut().column();
            let distance = self.distances.get((row, column)).unwrap_or(&0);

            String::from(format!(" {} ", distance))
        })
    }
}

impl BaseGrid for DistanceGrid {
    fn cell(&self, row: i32, column: i32) -> Option<&GridCell> {
        self.grid.cell(row, column)
    }

    fn iter(&self) -> GridIterator {
        self.grid.iter()
    }

    fn each_row(&self) -> std::slice::Iter<'_, Vec<GridCell>> {
        self.grid.each_row()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_distance_grid() {
        let rows = 3;
        let columns = 3;
        let distance_grid = DistanceGrid::new(rows, columns);

        assert_eq!(distance_grid.grid, Grid::new(3, 3));
        assert_eq!(distance_grid.distances.get((0, 0)), Some(&0));
    }

    #[test]
    fn test_display_distance_grid() {
        let rows = 3;
        let columns = 3;
        let mut distance_grid = DistanceGrid::new(rows, columns);
        let display = distance_grid.display();

        assert_eq!(
            display
                .to_string()
                .replace(" ", "")
                .replace("\n", "")
                .trim(),
            "+---+---+---+
             | 0 | 0 | 0 |
             +---+---+---+
             | 0 | 0 | 0 |
             +---+---+---+
             | 0 | 0 | 0 |
             +---+---+---+"
                .replace(" ", "")
                .replace("\n", "")
                .trim(),
        );
    }
}
