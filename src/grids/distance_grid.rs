use super::{
    base_grid::{BaseGrid, GridCell, GridIterator},
    distances::Distances,
    grid::{Grid, GridDisplay},
};

pub struct DistanceGrid {
    grid: Grid,
}

impl DistanceGrid {
    pub fn new(rows: i32, columns: i32) -> Self {
        Self {
            grid: Grid::new(rows, columns),
        }
    }

    pub fn display(&self) -> GridDisplay<'_> {
        let root = self.cell(0, 0).unwrap();
        let mut distances = Distances::new((root.borrow().row(), root.borrow().column()));
        distances.calculate(root.clone(), &self.grid);

        GridDisplay::new(&self.grid, |_| String::from(" 1 "))
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

mod tests {
    use super::DistanceGrid;
    use crate::algorithms::binary_tree::BinaryTree;

    #[test]
    fn new_distance_grid() {
        let mut grid = DistanceGrid::new(3, 3);
        BinaryTree::on(&mut grid);

        println!("{}", grid.display())
    }
}
