mod algorithms;
mod grids;
mod utils;

use algorithms::{
    aldous_broder::AldousBroder, binary_tree::BinaryTree, hunt_and_kill::HuntAndKill,
    recursive_backtracker::RecursiveBacktracker, sidewinder::Sidewinder, wilsons::Wilsons,
    Algorithms, Apply,
};
use grids::{base_grid::BaseGrid, distance_grid::DistanceGrid, grid::Grid};
use inquire::Select;
use std::fmt::Display;

#[derive(PartialEq)]
enum YesNo {
    Yes,
    No,
}

impl Display for YesNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YesNo::Yes => write!(f, "Yes"),
            YesNo::No => write!(f, "No"),
        }
    }
}

fn main() {
    let algorithms = vec![
        Algorithms::BinaryTree(BinaryTree::new()),
        Algorithms::Sidewinder(Sidewinder::new()),
        Algorithms::AldousBroder(AldousBroder::new()),
        Algorithms::Wilsons(Wilsons::new()),
        Algorithms::HuntAndKill(HuntAndKill::new()),
        Algorithms::RecursiveBacktracker(RecursiveBacktracker::new()),
    ];

    let algorithm = Select::new(
        "Please choose an algorithm to generate the maze:",
        algorithms,
    )
    .prompt()
    .unwrap();

    let with_distance = Select::new(
        "Would you like to show the distance from the north west cell?",
        vec![YesNo::Yes, YesNo::No],
    )
    .prompt()
    .unwrap();

    let mut grid: Box<dyn BaseGrid> = if with_distance == YesNo::Yes {
        Box::new(DistanceGrid::new(6, 6))
    } else {
        Box::new(Grid::new(6, 6))
    };

    algorithm.apply(&mut *grid);
    println!("{}", (grid).display());
}
