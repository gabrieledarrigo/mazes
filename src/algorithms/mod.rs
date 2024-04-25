use self::{
    aldous_broder::AldousBroder, binary_tree::BinaryTree, hunt_and_kill::HuntAndKill,
    recursive_backtracker::RecursiveBacktracker, sidewinder::Sidewinder, wilsons::Wilsons,
};
use crate::grids::base_grid::BaseGrid;
use std::fmt::Display;

pub mod aldous_broder;
pub mod binary_tree;
pub mod hunt_and_kill;
pub mod recursive_backtracker;
pub mod sidewinder;
pub mod wilsons;

pub trait On {
    fn on(&self, grid: &mut dyn BaseGrid);
}

pub trait Apply {
    fn apply(&self, grid: &mut dyn BaseGrid);
}

pub enum Algorithms {
    BinaryTree(BinaryTree),
    Sidewinder(Sidewinder),
    AldousBroder(AldousBroder),
    Wilsons(Wilsons),
    HuntAndKill(HuntAndKill),
    RecursiveBacktracker(RecursiveBacktracker),
}

impl Apply for Algorithms {
    fn apply(&self, grid: &mut dyn BaseGrid) {
        match self {
            Algorithms::BinaryTree(algorithm) => algorithm.on(grid),
            Algorithms::Sidewinder(algorithm) => algorithm.on(grid),
            Algorithms::AldousBroder(algorithm) => algorithm.on(grid),
            Algorithms::Wilsons(algorithm) => algorithm.on(grid),
            Algorithms::HuntAndKill(algorithm) => algorithm.on(grid),
            Algorithms::RecursiveBacktracker(algorithm) => algorithm.on(grid),
        }
    }
}

impl Display for Algorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithms::BinaryTree(_) => write!(f, "Binary Tree"),
            Algorithms::Sidewinder(_) => write!(f, "Sidewinder"),
            Algorithms::AldousBroder(_) => write!(f, "Aldous Broder"),
            Algorithms::Wilsons(_) => write!(f, "Wilsons"),
            Algorithms::HuntAndKill(_) => write!(f, "Hunt And Kill"),
            Algorithms::RecursiveBacktracker(_) => write!(f, "Recursive Backtracker"),
        }
    }
}
