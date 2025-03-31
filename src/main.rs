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
            println!("迷路の生成に失敗しました。もう一度試してください。");
            return;
        }
    };
    
    let mut game_clear = false;
    
    while !game_clear {
        // 迷路を表示
        maze.display();
        
        print!("移動方向を入力してください (w/a/s/d/q): ");
        io::stdout().flush().unwrap();
        
        // ユーザー入力を受け取る
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let direction = input.trim().to_lowercase();
        
        if direction == "q" {
            println!("ゲームを終了します。");
            break;
        }
        
        // プレイヤーを移動
        game_clear = maze.move_player(&direction);
        
        if game_clear {
            maze.display();
            println!("おめでとうございます！ゴールに到達しました！");
        }
    }
}
