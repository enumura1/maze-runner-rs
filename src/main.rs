use std::io::{self, Write};
use std::env;
use maze_runner_rs::maze::Maze;

fn main() {
    println!("Welcome to the text-based maze game!");
    println!("Controls: w=up, s=down, a=left, d=right, q=quit");
    
    // Parse command line arguments for width and height
    let args: Vec<String> = env::args().collect();
    
    // Default values
    let mut width = 17;
    let mut height = 11;
    
    // Parse width if provided
    if args.len() > 1 {
        if let Ok(w) = args[1].parse::<usize>() {
            width = w;
            println!("Using custom width: {}", width);
        }
    }
    
    // Parse height if provided
    if args.len() > 2 {
        if let Ok(h) = args[2].parse::<usize>() {
            height = h;
            println!("Using custom height: {}", height);
        }
    }
    
    // Create a new maze with error handling
    let mut maze = match Maze::with_size(width, height) {
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
        game_clear = maze.move_player(&direction);
        
        if game_clear {
            maze.display();
            println!("Congratulations! You've reached the goal!");
        }
    }
}
