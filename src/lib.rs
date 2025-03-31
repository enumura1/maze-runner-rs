/// A simple text-based maze game library.
/// 
/// This library provides functionality to create and play maze games
/// with procedurally generated mazes.

pub mod maze;
pub mod generator;
pub mod position;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_maze_creation() {
        let maze = maze::Maze::create();
        assert!(maze.is_some());
    }
    
    #[test]
    fn test_player_movement() {
        if let Some(mut maze) = maze::Maze::create() {
            // 移動前の位置を保存
            let initial_pos = maze.get_state().player_position;
            
            // 右に移動を試みる（壁でなければ成功するはず）
            let moved = maze.try_move("d");
            
            // 移動後の位置を取得
            let new_pos = maze.get_state().player_position;
            
            // 移動が成功した場合、位置が変わっているはず
            if moved {
                assert_ne!(initial_pos.x, new_pos.x);
            }
        } else {
            panic!("Failed to create maze for test");
        }
    }
}
