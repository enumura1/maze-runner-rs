# Maze Game Library

A simple Rust library for creating and playing text-based maze games. The maze is procedurally generated, and the player must find their way from the entrance to the exit.

## Features

- Randomly generated mazes
- Simple text-based UI
- Keyboard controls for character movement
- Can be used as a library or standalone game
- Customizable maze dimensions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
maze_game = "0.1.0"
```

## Usage as a Library

```rust
use maze_game::maze::Maze;

fn main() {
    // Create a new maze with default size (17x11)
    let mut maze = match Maze::new() {
        Some(m) => m,
        None => {
            println!("Failed to generate the maze.");
            return;
        }
    };

    // Or create a maze with custom size (width, height)
    let mut custom_maze = match Maze::with_size(25, 15) {
        Some(m) => m,
        None => {
            println!("Failed to generate the maze.");
            return;
        }
    };

    // Display the maze
    println!("{}", maze.get_maze_as_string());

    // Try to move the player, Move right
    let moved = maze.move_player("d");
    
    // Check if the game is completed
    let state = maze.get_state();
    if state.is_completed {
        println!("Congratulations! You've reached the goal!");
    }
}
```

## Playing the Game

Execute the following command to run the standalone game with default size (17x11):
```
cargo run
```

To specify custom maze dimensions:
```
cargo run -- 25 15
```
Where 25 is the width and 15 is the height.

### Controls

- `w` or up: Move up
- `s` or down: Move down
- `a` or left: Move left
- `d` or right: Move right
- `q`: Quit the game

## Project Structure

- `lib.rs`: Public module exports
- `main.rs`: Standalone game loop and user input handling
- `maze.rs`: Maze data structure and operations
- `generator.rs`: Maze generation algorithms
- `position.rs`: Position structure

## License

This project is licensed under the MIT License - see the LICENSE file for details.
