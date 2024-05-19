# Mazes for Programmer

[![Rust](https://github.com/gabrieledarrigo/mazes/actions/workflows/build.yml/badge.svg)](https://github.com/gabrieledarrigo/mazes/actions/workflows/build.yml)

A Rust version of Jamis Buck's "[Mazes for Programmer](http://www.mazesforprogrammers.com/)" algorithms.

## Usage

To run the application, clone the repository and run the following command:

```shell
$ cargo run
```

The application will show an interactive prompt to choose: the algorithm to generate the maze, the size of the grid, if you want to display the distances from the northwest corner, and if you want to display the maze with colours.

![example](https://github.com/gabrieledarrigo/mazes/assets/1985555/ce2b1192-ba41-4fb7-b401-1b2ca54bdc11)


## Algorithms

Every algorithm generates a maze by working on a grid of cells, where each cell can be a wall, the boundary between cells, or a passage. 

### Binary Tree

The Binary Tree algorithm is the simplest one. It works by iterating over each cell of the grid and randomly choosing to link the cell with the one to the north or the one to the east.

### Sidewinder

The Sidewinder algorithm is a variation of the Binary Tree algorithm.   
It works by iterating over each row of the grid; for each cell, it randomly chooses to link the east cell, and add it to the list of visited cells, or to choose a cell from the set of visited ones and link it to the south.

### Aldous-Broder

The Aldous-Broder algorithm is a random walk algorithm, developed by David Aldous and Andrei Broder. A cell and a random neighbour are chosen; if the neighbour is not visited, the two cells are linked, otherwise, the neighbour becomes the current cell, and a new neighbour is chosen.

### Wilson

The Wilson algorithm is a loop-erased random walk algorithm, developed by David Wilson. It first chooses a random cell, and another cell as the start cell. 
Then, the algorithm performs a random walk until a visited cell is reached. 
If a loop is formed, the walk is erased and the algorithm continues from the point before it.

### Hunt-and-Kill

The Hunt-and-Kill algorithm performs a random walk, but unlike the Aldous-Broder algorithm, a cell cannot be visited twice.
If the walk reaches a dead end, the algorithm starts a new walk from the top, scanning the grid left to right until it finds an unvisited cell from which it can continue the walk.

### Recursive Backtracker

The Recursive Backtracker algorithm is a depth-first search algorithm. It starts from a random cell and explores as far as possible along each branch before backtracking.
