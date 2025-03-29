mod position;
mod generator;
mod maze;

use std::io::{self, Write};
use maze::Maze;

fn main() {
    println!("テキストベース迷路ゲームへようこそ！");
    println!("操作方法: w=上, s=下, a=左, d=右, q=終了");
    
    // 迷路を生成（エラーハンドリング付き）
    let mut maze = match Maze::new() {
        Some(m) => m,
        None => {
            println!("迷路の生成に失敗しました。もう一度試してください。");
            return;
        }
    };
    
    let mut game_over = false;
    
    while !game_over {
        // 迷路を表示
        maze.display();
        
        // ユーザー入力を受け取る
        print!("移動方向を入力してください (w/a/s/d/q): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let direction = input.trim().to_lowercase();
        
        if direction == "q" {
            println!("ゲームを終了します。");
            break;
        }
        
        // プレイヤーを移動
        game_over = maze.move_player(&direction);
        
        if game_over {
            maze.display();
            println!("おめでとうございます！ゴールに到達しました！");
        }
    }
}
