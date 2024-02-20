use crate::grids::grid::Grid;
use rand::Rng;

pub struct Sidewinder {}

impl Sidewinder {
    pub fn on(grid: &mut Grid) {
        let mut rng = rand::thread_rng();

        for row in grid.each_row() {
            let mut run = vec![];

            for cell in row {
                run.push(cell);

                let should_close = {
                    let cell = cell.borrow_mut();
                    let at_eastern_boundary = cell.east().is_none();
                    let at_northern_boundary = cell.north().is_none();

                    at_eastern_boundary || at_northern_boundary == false && rng.gen_bool(0.5)
                };

                if should_close {
                    let index = rng.gen_range(0..run.len());
                    let mut member = run[index].borrow_mut();

                    if let Some((northern_row, norther_column)) = member.north() {
                        let north = grid.cell(northern_row, norther_column).unwrap().to_owned();
                        member.link(north);
                    }
                } else {
                    let mut cell = cell.borrow_mut();

                    if let Some((eastern_row, eastern_column)) = cell.east() {
                        let east = grid.cell(eastern_row, eastern_column).unwrap().to_owned();
                        cell.link(east);
                    }
                }
            }
        }
    }
}
