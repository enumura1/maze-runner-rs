# Rust Maze Game

A simple text-based maze game. The maze is procedurally generated, and the player must find their way from the entrance to the exit.

## Features

- Randomly generated mazes
- Simple text-based UI
- Keyboard controls for character movement
- Reach the goal to win

## Controls

- `w` or up: Move up
- `s` or down: Move down
- `a` or left: Move left
- `d` or right: Move right
- `q`: Quit the game

## How to Run

Execute the following command to run the game.
```
cargo run
```

## Project Structure

- `main.rs`: Main game loop and user input handling
- `maze.rs`: Maze data structure and operations
- `generator.rs`: Maze generation algorithms
- `position.rs`: Position structure

## Algorithm

The maze is generated using a "recursive backtracking" algorithm (also known as the "digging method"). This algorithm works as follows:

1. Fill the entire maze with walls
2. Start at a specific point and mark it as a passage
3. Choose a random direction and, if the cell two spaces away is a wall, convert it and the wall between to passages
4. Recursively repeat step 3 from the newly created passage

This creates a complex maze with dead ends and branching paths.
