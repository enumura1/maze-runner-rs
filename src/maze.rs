use crate::position::Position;
use crate::generator::{dig_maze, find_path_position, find_path_position_from_bottom};

/// Character representing walls
pub const WALL: char = '#';
/// Character representing paths
pub const PATH: char = ' ';
/// Character representing the player
pub const PLAYER: char = 'P';
/// Character representing the goal
pub const GOAL: char = 'G';

/// A struct representing the maze game state
pub struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
    player: Position,
    goal: Position,
}

/// State of the maze game that can be queried by library users
pub struct MazeState {
    /// Current position of the player
    pub player_position: Position,
    /// Position of the goal
    pub goal_position: Position,
    /// Whether the player has reached the goal
    pub is_completed: bool,
}

impl Maze {
    /// Creates a new maze with a player and goal with default size (17x11).
    /// Returns None if maze generation fails.
    pub fn new() -> Option<Self> {
        Self::with_size(17, 11)
    }
    
    /// Creates a new maze with a player and goal with custom size.
    /// Returns None if maze generation fails.
    /// 
    /// # Arguments
    /// * `width` - The width of the maze (must be odd and >= 5)
    /// * `height` - The height of the maze (must be odd and >= 5)
    pub fn with_size(width: usize, height: usize) -> Option<Self> {
        // Ensure minimum size and odd dimensions for proper maze generation
        let width = if width < 5 { 5 } else { width };
        let height = if height < 5 { 5 } else { height };
        
        // Ensure odd dimensions (needed for the maze generation algorithm)
        let width = if width % 2 == 0 { width + 1 } else { width };
        let height = if height % 2 == 0 { height + 1 } else { height };
        
        let mut rng = rand::rng();
        let mut grid = vec![vec![WALL; width]; height];
        
        // Generate the maze using the recursive backtracking algorithm
        dig_maze(&mut grid, width, height, 1, 1, &mut rng);
        
        // Set the player's initial position (a path near the top-left)
        let player = match find_path_position(&grid, width, height, 1, 1) {
            Some(pos) => pos,
            None => return None,
        };
        
        // Set the goal position (a path near the bottom-right, different from the player)
        let goal = match find_path_position_from_bottom(&grid, width, height, width - 2, height - 2, &player) {
            Some(pos) => pos,
            None => return None,
        };
        
        // Create the maze and place player and goal
        let mut maze = Maze { width, height, grid, player, goal };
        maze.update_grid();
        
        Some(maze)
    }
    
    /// Update the grid with player and goal positions
    fn update_grid(&mut self) {
        // Reset the grid (remove player and goal)
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == PLAYER || self.grid[y][x] == GOAL {
                    self.grid[y][x] = PATH;
                }
            }
        }
        
        // Place player and goal
        self.grid[self.player.y][self.player.x] = PLAYER;
        self.grid[self.goal.y][self.goal.x] = GOAL;
    }
    
    /// Display the maze in the console
    pub fn display(&self) {
        println!("\nMaze: (P=Player, G=Goal, #=Wall)\n");
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.grid[y][x]);
            }
            println!();
        }
        println!();
    }
    
    /// Get the maze as a string
    pub fn get_maze_as_string(&self) -> String {
        let mut result = String::new();
        result.push_str("Maze: (P=Player, G=Goal, #=Wall)\n\n");
        
        for y in 0..self.height {
            for x in 0..self.width {
                result.push(self.grid[y][x]);
            }
            result.push('\n');
        }
        
        result
    }
    
    /// Move the player in the specified direction
    /// Returns true if the goal is reached, false otherwise
    pub fn move_player(&mut self, direction: &str) -> bool {
        let mut new_x = self.player.x;
        let mut new_y = self.player.y;
        
        match direction {
            "w" | "up" => {
                if new_y > 0 {
                    new_y -= 1;
                }
            }
            "s" | "down" => {
                if new_y < self.height - 1 {
                    new_y += 1;
                }
            }
            "a" | "left" => {
                if new_x > 0 {
                    new_x -= 1;
                }
            }
            "d" | "right" => {
                if new_x < self.width - 1 {
                    new_x += 1;
                }
            }
            _ => return false,
        }
        
        // Move if not a wall
        if self.grid[new_y][new_x] != WALL {
            self.player.x = new_x;
            self.player.y = new_y;
            self.update_grid();
            
            // Check if goal is reached
            return self.player.x == self.goal.x && self.player.y == self.goal.y;
        }
        
        false
    }
    
    /// Get the current state of the maze
    pub fn get_state(&self) -> MazeState {
        MazeState {
            player_position: self.player,
            goal_position: self.goal,
            is_completed: self.player.x == self.goal.x && self.player.y == self.goal.y,
        }
    }
    
    /// Get the width of the maze
    pub fn get_width(&self) -> usize {
        self.width
    }
    
    /// Get the height of the maze
    pub fn get_height(&self) -> usize {
        self.height
    }
}
