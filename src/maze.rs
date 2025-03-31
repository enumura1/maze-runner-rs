use crate::position::Position;
use crate::generator::{dig_maze, find_path_position, find_path_position_from_bottom};

// 迷路の設定
pub const WIDTH: usize = 15;
pub const HEIGHT: usize = 9;
pub const WALL: char = '#';
pub const PATH: char = ' ';
pub const PLAYER: char = 'P';
pub const GOAL: char = 'G';

// 迷路のゲーム状態を表す構造体
pub struct Maze {
    grid: [[char; WIDTH]; HEIGHT],
    player: Position,
    goal: Position,
}

impl Maze {
    // 新しい迷路を生成する
    pub fn new() -> Option<Self> {
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
            // 通路が見つからない場合
            None => return None,
        };
        
        // ゴールの位置を設定（右下の通路、プレイヤーと異なる位置）
        let goal = match find_path_position_from_bottom(&grid, WIDTH - 2, HEIGHT - 2, &player) {
            Some(pos) => pos,
            // 通路が見つからない場合
            None => return None,
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
    pub fn display(&self) {
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
