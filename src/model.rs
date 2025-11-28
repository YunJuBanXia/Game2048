pub struct Game2048 {
    pub board: [[u32; 4]; 4],
    pub score: u32,
}

impl Game2048 {
    pub fn new() -> Self {
        Game2048 {
            board: [[0; 4]; 4],
            score: 0,
        }
    }

    pub fn reset(&mut self) {
        self.board = [[0; 4]; 4];
        self.score = 0;
    }

    pub fn is_lose(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                // 检测是否有空位
                if self.board[i][j] == 0 {
                    return false;
                }
                
                // 检测相邻格子能否合并
                if j < 3 && self.board[i][j] == self.board[i][j + 1] {
                    return false;
                }
                if i < 3 && self.board[i][j] == self.board[i + 1][j] {
                    return false;
                }
            }
        }
        true
    }
}