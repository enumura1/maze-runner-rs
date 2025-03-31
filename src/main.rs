mod position;
mod generator;
mod maze;

use std::io::{self, Write};
use maze::Maze;

fn main() {
    println!("Welcome to the text-based maze game!");
    println!("Instructions: w=up, s=down, a=left, d=right, q=end");
    
    // 迷路を生成（エラーハンドリング付き）
    let mut maze = match Maze::new() {
        Some(m) => m,
        None => {
            println!("Failed to generate the maze. Please try again.");
            return;
        }
    };
    
    let mut game_clear = false;
    
    while !game_clear {
        // 迷路を表示
        maze.display();
        
        print!("Enter your move (w/a/s/d/q): ");
        io::stdout().flush().unwrap();
        
        // ユーザー入力を受け取る
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let direction = input.trim().to_lowercase();
        
        if direction == "q" {
            println!("Game terminated. ");
            break;
        }
        
        // プレイヤーを移動
        game_clear = maze.move_player(&direction);
        
        if game_clear {
            maze.display();
            println!("Congratulations! You've reached the goal!");
        }
    }
}
