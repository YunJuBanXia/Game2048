pub struct Game2048 {
    pub board: [[u32; 4]; 4],
    pub score: u32,  // score 是最大数
}

impl Game2048 {
    pub fn new() -> Self {
        Game2048 {
            board: [[0; 4]; 4],
            score: 0,
        }
    }

    pub fn get_empty_positions(&self) -> Vec<(usize, usize)> {
        let mut empty_positions = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == 0 {
                    empty_positions.push((i, j));
                }
            }
        }
        empty_positions
    }
} 