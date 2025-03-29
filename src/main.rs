use std::io::{self, Write};

fn main() {
    const WIDTH: usize = 5;
    const HEIGHT: usize = 5;
    
    const WALL: char = '#';
    const PATH: char = ' ';
    const PLAYER: char = 'P';
    
    // プレイヤーの位置
    let mut player_x: usize = 2;
    let mut player_y: usize = 2;
    
    // 迷路を初期化
    let mut maze = [[WALL; WIDTH]; HEIGHT];
    
    // 中央の十字に通路を作る
    maze[2][1] = PATH;
    maze[2][2] = PATH;
    maze[2][3] = PATH;
    maze[1][2] = PATH;
    maze[3][2] = PATH;
    
    // ゲームループの開始
    println!("簡単な迷路ゲームへようこそ！");
    println!("w=上、s=下、a=左、d=右、q=終了");
    
    let mut running = true;
    while running {
        // 迷路をリセット（プレイヤーを一旦削除）
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if maze[y][x] == PLAYER {
                    maze[y][x] = PATH;
                }
            }
        }
        
        // プレイヤーを新しい位置に配置
        maze[player_y][player_x] = PLAYER;
        
        // 迷路を表示
        println!("\n現在の迷路：");
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", maze[y][x]);
            }
            println!();
        }
        
        // ユーザー入力を受け取る
        print!("移動方向を入力してください (w/a/s/d/q): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let direction = input.trim().to_lowercase();
        
        // 入力に基づいてプレイヤーを移動
        match direction.as_str() {
            "w" => {
                if player_y > 0 && maze[player_y - 1][player_x] != WALL {
                    player_y -= 1;
                }
            },
            "s" => {
                if player_y < HEIGHT - 1 && maze[player_y + 1][player_x] != WALL {
                    player_y += 1;
                }
            },
            "a" => {
                if player_x > 0 && maze[player_y][player_x - 1] != WALL {
                    player_x -= 1;
                }
            },
            "d" => {
                if player_x < WIDTH - 1 && maze[player_y][player_x + 1] != WALL {
                    player_x += 1;
                }
            },
            "q" => {
                println!("ゲームを終了します。");
                running = false;
            },
            _ => println!("無効な入力です。w/a/s/d/qのいずれかを入力してください。"),
        }
    }
}
