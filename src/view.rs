use crate::model::Game2048;

impl Game2048 {
    pub fn display(&self) {
        for row in &self.board {
            println!("-------------------------");
            for &element in row {
                if element == 0 {
                    print!("|{:^5}", " ");
                }
                else {
                    print!("|{:^5}", element);
                }
            }
            println!("|");
        }
        println!("-------------------------");
    }
}