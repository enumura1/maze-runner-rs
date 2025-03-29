use rand::Rng;
use rand::seq::SliceRandom;
use crate::maze::{WALL, PATH, WIDTH, HEIGHT};
use crate::position::Position;

// 迷路生成のための穴掘り法
pub fn dig_maze(grid: &mut [[char; WIDTH]; HEIGHT], start_x: usize, start_y: usize, rng: &mut impl Rng) {
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

// 通路の位置を見つける補助関数（左上から探索）
pub fn find_path_position(grid: &[[char; WIDTH]; HEIGHT], start_x: usize, start_y: usize) -> Option<Position> {
    // 指定された位置が通路なら、その位置を返す
    if grid[start_y][start_x] == PATH {
        return Some(Position::new(start_x, start_y));
    }
    
    // 通路を探す（左上から右下に）
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if grid[y][x] == PATH {
                return Some(Position::new(x, y));
            }
        }
    }
    
    // 見つからない場合
    None
}

// 通路の位置を見つける補助関数（右下から探索、特定の位置を避ける）
pub fn find_path_position_from_bottom(
    grid: &[[char; WIDTH]; HEIGHT], 
    start_x: usize, 
    start_y: usize,
    avoid: &Position
) -> Option<Position> {
    // 指定された位置が通路かつ避けるべき位置でなければ、その位置を返す
    if grid[start_y][start_x] == PATH && !(start_x == avoid.x && start_y == avoid.y) {
        return Some(Position::new(start_x, start_y));
    }
    
    // 通路を探す（右下から左上に）
    for y in (0..HEIGHT).rev() {
        for x in (0..WIDTH).rev() {
            if grid[y][x] == PATH && !(x == avoid.x && y == avoid.y) {
                return Some(Position::new(x, y));
            }
        }
    }
    
    // 見つからない場合（最後の手段として、別の通路を探す）
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if grid[y][x] == PATH && !(x == avoid.x && y == avoid.y) {
                return Some(Position::new(x, y));
            }
        }
    }
    
    // それでも見つからない場合
    None
}
