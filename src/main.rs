use std::io::{self, Write};
use rand::seq::SliceRandom;
use rand::Rng;

// 迷路の設定
const WIDTH: usize = 15;
const HEIGHT: usize = 10;
const WALL: char = '#';
const PATH: char = ' ';
const PLAYER: char = 'P';
const GOAL: char = 'G';

// プレイヤーの位置を表す構造体
struct Position {
    x: usize,
    y: usize,
}

// 迷路のゲーム状態を表す構造体
struct Maze {
    grid: [[char; WIDTH]; HEIGHT],
    player: Position,
    goal: Position,
}

impl Maze {
    // 新しい迷路を生成する
    fn new() -> Option<Self> {
        let mut rng = rand::rng();
        let mut grid = [[WALL; WIDTH]; HEIGHT];
        
        // まず迷路全体を壁にする
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                grid[y][x] = WALL;
            }
        }
        
        // 簡易的な迷路生成アルゴリズム（穴掘り法）
        dig_maze(&mut grid, 1, 1, &mut rng);
        
        // プレイヤーの初期位置を設定（左上の通路）
        let player = match find_path_position(&grid, 1, 1) {
            Some(pos) => pos,
            None => return None, // 通路が見つからない場合はNoneを返す
        };
        
        // ゴールの位置を設定（右下の通路）
        let goal = match find_path_position(&grid, WIDTH - 2, HEIGHT - 2) {
            Some(pos) => pos,
            None => return None, // 通路が見つからない場合はNoneを返す
        };
        
        // プレイヤーとゴールを配置
        let mut maze = Maze { grid, player, goal };
        maze.update_grid();
        
        Some(maze)
    }
    
    // グリッドにプレイヤーとゴールを配置する
    fn update_grid(&mut self) {
        // グリッドをリセット（プレイヤーとゴールを削除）
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.grid[y][x] == PLAYER || self.grid[y][x] == GOAL {
                    self.grid[y][x] = PATH;
                }
            }
        }
        
        // プレイヤーとゴールを配置
        self.grid[self.player.y][self.player.x] = PLAYER;
        self.grid[self.goal.y][self.goal.x] = GOAL;
    }
    
    // 迷路を表示する
    fn display(&self) {
        println!("\n迷路: (P=プレイヤー, G=ゴール, #=壁)\n");
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", self.grid[y][x]);
            }
            println!();
        }
        println!();
    }
    
    // プレイヤーを移動する
    fn move_player(&mut self, direction: &str) -> bool {
        let mut new_x = self.player.x;
        let mut new_y = self.player.y;
        
        match direction {
            "w" | "up" => {
                if new_y > 0 {
                    new_y -= 1;
                }
            }
            "s" | "down" => {
                if new_y < HEIGHT - 1 {
                    new_y += 1;
                }
            }
            "a" | "left" => {
                if new_x > 0 {
                    new_x -= 1;
                }
            }
            "d" | "right" => {
                if new_x < WIDTH - 1 {
                    new_x += 1;
                }
            }
            _ => return false,
        }
        
        // 壁でなければ移動
        if self.grid[new_y][new_x] != WALL {
            self.player.x = new_x;
            self.player.y = new_y;
            self.update_grid();
            
            // ゴールに到達したかチェック
            return self.player.x == self.goal.x && self.player.y == self.goal.y;
        }
        
        false
    }
}

// 迷路生成のための穴掘り法
fn dig_maze(grid: &mut [[char; WIDTH]; HEIGHT], start_x: usize, start_y: usize, rng: &mut impl Rng) {
    // 開始位置を通路にする
    grid[start_y][start_x] = PATH;
    
    // 掘る方向をランダムに決める
    let directions = [(0, -2), (2, 0), (0, 2), (-2, 0)];
    let mut shuffled_directions = directions.to_vec();
    shuffled_directions.shuffle(rng);
    
    // 各方向に掘っていく
    for (dx, dy) in shuffled_directions {
        let nx = start_x as isize + dx;
        let ny = start_y as isize + dy;
        
        // 範囲内かつ掘れる場所かチェック
        if nx > 0 && nx < WIDTH as isize - 1 && ny > 0 && ny < HEIGHT as isize - 1 
            && grid[ny as usize][nx as usize] == WALL {
            // 間の壁も掘る
            grid[(start_y as isize + dy / 2) as usize][(start_x as isize + dx / 2) as usize] = PATH;
            // 再帰的に掘り進める
            dig_maze(grid, nx as usize, ny as usize, rng);
        }
    }
}

// 通路の位置を見つける補助関数
fn find_path_position(grid: &[[char; WIDTH]; HEIGHT], start_x: usize, start_y: usize) -> Option<Position> {
    // 指定された位置が通路なら、その位置を返す
    if grid[start_y][start_x] == PATH {
        return Some(Position { x: start_x, y: start_y });
    }
    
    // 通路を探す
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if grid[y][x] == PATH {
                return Some(Position { x, y });
            }
        }
    }
    
    // 見つからない場合（通常ここには来ない）
    None
}

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
