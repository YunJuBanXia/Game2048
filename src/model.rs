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
} 