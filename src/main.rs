use std::io::{self, Write};
use maze_game::maze::Maze;

fn main() {
    println!("Welcome to the text-based maze game!");
    println!("Controls: w=up, s=down, a=left, d=right, q=quit");
    
    // Create a new maze with error handling
    let mut maze = match Maze::create() {
        Some(m) => m,
        None => {
            println!("Failed to generate the maze. Please try again.");
            return;
        }
    };
    
    let mut game_clear = false;
    
    while !game_clear {
        // Display the maze
        maze.display();
        
        print!("Enter your move (w/a/s/d/q): ");
        io::stdout().flush().unwrap();
        
        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let direction = input.trim().to_lowercase();
        
        if direction == "q" {
            println!("Game terminated.");
            break;
        }
        
        // Move the player
        game_clear = maze.try_move(&direction);
        
        if game_clear {
            maze.display();
            println!("Congratulations! You've reached the goal!");
        }
    }
}
