// 通过crossterm获取键盘输入
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crate::model::Game2048;
use rand::Rng;

impl Game2048 {
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

    pub fn get_input(&self) -> Option<KeyCode> {
        if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
            Some(code)
        } else {
            None
        }
    }

    pub fn move_up(&mut self) -> () {
        for column in 0..4 {
            let mut temp = Vec::new();
            for row in 0..4 {
                if self.board[row][column] != 0 {
                    temp.push(self.board[row][column]);
                }
            }
            for i in 0..temp.len() - 1 {
                if temp[i] == temp[i + 1] {
                    temp[i] *= 2;
                    self.score = if self.score + temp[i] > self.score {
                        temp[i]
                    } else {
                        self.score
                    };
                    temp.remove(i + 1);
                }
            }
            for i in 0..4 {
                self.board[i][column] = if i < temp.len() { temp[i] } else { 0 };
            }
        }
    }

    pub fn move_down(&mut self) -> () {
        for column in 0..4 {
            let mut temp = Vec::new();
            for row in (0..4).rev() {
                if self.board[row][column] != 0 {
                    temp.push(self.board[row][column]);
                }
            }
            for i in 0..temp.len() - 1 {
                if temp[i] == temp[i + 1] {
                    temp[i] *= 2;
                    self.score = if self.score + temp[i] > self.score {
                        temp[i]
                    } else {
                        self.score
                    };
                    temp.remove(i + 1);
                }
            }
            for i in 0..4 {
                self.board[3 - i][column] = if i < temp.len() { temp[i] } else { 0 };
            }
        }
    }

    pub fn move_left(&mut self) -> () {
        for row in 0..4 {
            let mut temp = Vec::new();
            for column in 0..4 {
                if self.board[row][column] != 0 {
                    temp.push(self.board[row][column]);
                }
            }
            for i in 0..temp.len() - 1 {
                if temp[i] == temp[i + 1] {
                    temp[i] *= 2;
                    self.score = if self.score + temp[i] > self.score {
                        temp[i]
                    } else {
                        self.score
                    };
                    temp.remove(i + 1);
                }
            }
            for i in 0..4 {
                self.board[row][i] = if i < temp.len() { temp[i] } else { 0 };
            }
        }
    }

    pub fn move_right(&mut self) -> () {
        for row in 0..4 {
            let mut temp = Vec::new();
            for column in (0..4).rev() {
                if self.board[row][column] != 0 {
                    temp.push(self.board[row][column]);
                }
            }
            for i in 0..temp.len() - 1 {
                if temp[i] == temp[i + 1] {
                    temp[i] *= 2;
                    self.score = if self.score + temp[i] > self.score {
                        temp[i]
                    } else {
                        self.score
                    };
                    temp.remove(i + 1);
                }
            }
            for i in 0..4 {
                self.board[row][3 - i] = if i < temp.len() { temp[i] } else { 0 };
            }
        }
    }

    // 上下左右移动
    pub fn make_move(&mut self, direction: KeyCode) -> (){
        match direction {
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            _ => (),
        }
    }

    pub fn add_randon_element(&mut self) {
        let mut rng = rand::thread_rng();
        let empty_positions = self.get_empty_positions();
        let (row, column) = if !empty_positions.is_empty() {
            let &(row, column) = empty_positions
                .get(rng.gen_range(0..empty_positions.len()))
                .unwrap();
            (row, column)
        } else {
            (4, 4) // Default value if no empty positions
        };
        if row == 4 && column == 4 {
            return;
        }
        let value = rng.gen_range(1..=2) * 2; // 2或4
        self.board[row][column] = value;
    }

    pub fn run(&mut self) {
        self.reset();
        loop {
            self.add_randon_element();
            self.display();
            if self.is_lose() {
                println!("Game Over! Your score is: {}", self.score);
                break;
            }
            let input = self.get_input();
            todo!()
        }
    }
}