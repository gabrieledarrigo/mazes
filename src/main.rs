mod algorithms;
mod grids;
mod utils;

use algorithms::{
    aldous_broder::AldousBroder, binary_tree::BinaryTree, hunt_and_kill::HuntAndKill,
    recursive_backtracker::RecursiveBacktracker, sidewinder::Sidewinder, wilsons::Wilsons,
    Algorithms, Apply,
};
use grids::{base_grid::BaseGrid, distance_grid::DistanceGrid, grid::Grid};
use inquire::{validator::Validation, Confirm, CustomType, Select};

pub const MIN_GRID_WIDTH: i32 = 5;
pub const MAX_GRID_WIDTH: i32 = 11;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    .prompt()?;

    let width: i32 = CustomType::new("Please choose the width of the grid:")
        .with_validator(|input: &i32| {
            if *input < MIN_GRID_WIDTH {
                return Ok(Validation::Invalid(
                    format!("Please enter a number greater than {}", MIN_GRID_WIDTH).into(),
                ));
            }

            if *input > MAX_GRID_WIDTH {
                return Ok(Validation::Invalid(
                    format!("Please enter a number lower than {}", MAX_GRID_WIDTH).into(),
                ));
            }

            Ok(Validation::Valid)
        })
        .with_help_message(
            format!(
                "Please enter a number between {} and {}",
                MIN_GRID_WIDTH, MAX_GRID_WIDTH
            )
            .as_str(),
        )
        .prompt()?;

    let with_distance =
        Confirm::new("Would you like to show the distance from the north west cell?")
            .with_default(false)
            .prompt()?;

    let with_colors = if with_distance {
        Confirm::new("Would you like to show the distance with colors?")
            .with_default(false)
            .prompt()?
    } else {
        false
    };

    let mut grid: Box<dyn BaseGrid> = if with_distance {
        Box::new(DistanceGrid::new(width, width))
    } else {
        Box::new(Grid::new(width, width))
    };

    algorithm.apply(&mut *grid);

    if with_colors {
        println!("\n\n{}", grid.display_with_color());
    } else {
        println!("\n\n{}", grid.display());
    }

    Ok(())
}
