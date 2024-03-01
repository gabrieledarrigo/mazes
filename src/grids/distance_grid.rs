use std::path::Display;

use super::{distances::Distances, grid::Grid};

pub struct DistanceGrid {
    distance: Option<Distances>,
    grid: Grid,
}

impl DistanceGrid {
    pub fn new(rows: i32, columns: i32) -> Self {
        Self {
            distance: None,
            grid: Grid::new(rows, columns),
        }
    }
}
