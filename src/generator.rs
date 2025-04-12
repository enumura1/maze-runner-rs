use rand::Rng;
use rand::seq::SliceRandom;
use crate::maze::{WALL, PATH};
use crate::position::Position;

/// Generates a maze using the recursive backtracking algorithm (digging method).
/// 
/// # Arguments
/// * grid - The grid to dig the maze in
/// * width - The width of the grid
/// * height - The height of the grid
/// * start_x - The starting X coordinate
/// * start_y - The starting Y coordinate
/// * rng - Random number generator
pub fn dig_maze(grid: &mut Vec<Vec<char>>, 
                width: usize, height: usize,
                start_x: usize, start_y: usize, 
                rng: &mut impl Rng) {
    // Mark starting position as a path
    grid[start_y][start_x] = PATH;

    // Choose random directions to dig
    let directions = [(0, -2), (2, 0), (0, 2), (-2, 0)];
    let mut shuffled_directions = directions.to_vec();
    shuffled_directions.shuffle(rng);   

    // Dig in each direction
    for (dx, dy) in shuffled_directions {
        let nx = start_x as isize + dx;
        let ny = start_y as isize + dy;

        // Check if within bounds and can dig
        if nx > 0 && nx < width as isize - 1 && ny > 0 && ny < height as isize - 1 
            && grid[ny as usize][nx as usize] == WALL {
            // Dig through the wall between positions
            grid[(start_y as isize + dy / 2) as usize][(start_x as isize + dx / 2) as usize] = PATH;
            // Continue digging recursively
            dig_maze(grid, width, height, nx as usize, ny as usize, rng);
        }
    }
}

/// Finds a path position from the top-left of the maze.
/// 
/// # Arguments
/// * grid - The maze grid
/// * width - The width of the grid
/// * height - The height of the grid
/// * start_x - The starting X coordinate to check
/// * start_y - The starting Y coordinate to check
/// 
/// # Returns
/// * Some(Position) - The found path position
/// * None - If no path position was found
pub fn find_path_position(grid: &Vec<Vec<char>>, 
    width: usize, height: usize,
    start_x: usize, start_y: usize) -> Option<Position> {
    // If the specified position is a path, return it
    if grid[start_y][start_x] == PATH {
        return Some(Position::new(start_x, start_y));
    }

    // Search for a path from top-left to bottom-right
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == PATH {
                return Some(Position::new(x, y));
            }
        }
    }

    // No path found
    None
}

/// Finds a path position from the bottom-right of the maze, avoiding a specific position.
/// 
/// # Arguments
/// * grid - The maze grid
/// * width - The width of the grid
/// * height - The height of the grid
/// * start_x - The starting X coordinate to check
/// * start_y - The starting Y coordinate to check
/// * avoid - Position to avoid (usually the player position)
/// 
/// # Returns
/// * Some(Position) - The found path position
/// * None - If no path position was found
pub fn find_path_position_from_bottom(
    grid: &Vec<Vec<char>>, 
    width: usize, height: usize,
    start_x: usize, 
    start_y: usize,
    avoid: &Position
) -> Option<Position> {
    // If the specified position is a path and not the position to avoid, return it
    if grid[start_y][start_x] == PATH && !(start_x == avoid.x && start_y == avoid.y) {
        return Some(Position::new(start_x, start_y));
    }

    // Search for a path from bottom-right to top-left, avoiding the specified position
    for y in (0..height).rev() {
        for x in (0..width).rev() {
            if grid[y][x] == PATH && !(x == avoid.x && y == avoid.y) {
                return Some(Position::new(x, y));
            }
        }
    }

    // No path found
    None
}
