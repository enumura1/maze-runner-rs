fn main() {
    const WIDTH: usize = 5;
    const HEIGHT: usize = 5;
    
    const WALL: char = '#';
    const PATH: char = ' ';
    const PLAYER: char = 'P';
    
    // プレイヤーの位置
    let player_x: usize = 2;
    let player_y: usize = 2;
    
    // 迷路を初期化
    let mut maze = [[WALL; WIDTH]; HEIGHT];
    
    // 中央の十字に通路を作る
    maze[2][1] = PATH;
    maze[2][2] = PATH;
    maze[2][3] = PATH;
    maze[1][2] = PATH;
    maze[3][2] = PATH;
    
    // プレイヤーを配置
    maze[player_y][player_x] = PLAYER;
    
    // 迷路を表示
    println!("プレイヤーのいる迷路：");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", maze[y][x]);
        }
        println!();
    }
}
