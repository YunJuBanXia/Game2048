// 通过crossterm获取键盘输入
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
use crate::model::Game2048;
use rand::Rng;
use std::thread;
use std::time::Duration;

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
        if let Event::Key(KeyEvent { code,kind: KeyEventKind::Press, .. }) = read().unwrap() {
            Some(code)
        } else {
            None
        }
    }

    pub fn move_up(&mut self) -> () {
        for column in 0..4 {
            let mut temp = Vec::new();
            let mut res = Vec::new();
            for row in 0..4 {
                if self.board[row][column] != 0 {
                    temp.push(self.board[row][column]);
                }
            }
            let mut index = 0;
            while index < temp.len() {
                if index + 1 < temp.len() && temp[index] == temp[index + 1] {
                    res.push(temp[index] * 2);
                    self.score += temp[index] * 2;
                    index += 2;
                } else {
                    res.push(temp[index]);
                    index += 1;
                }
            }
            while res.len() < 4 {
                res.push(0);
            }
            for i in 0..4 {
                self.board[i][column] = res[i];
            }
        }
    }

    pub fn move_down(&mut self) -> () {
        for column in 0..4 {
            let mut temp = Vec::new();
            let mut res = Vec::new();
            for row in (0..4).rev() {
                if self.board[row][column] != 0 {
                    temp.push(self.board[row][column]);
                }
            }
            let mut index = 0;
            while index < temp.len() {
                if index + 1 < temp.len() && temp[index] == temp[index + 1] {
                    res.push(temp[index] * 2);
                    self.score += temp[index] * 2;
                    index += 2;
                } else {
                    res.push(temp[index]);
                    index += 1;
                }
            }

            while res.len() < 4 {
                res.push(0);
            }
            
            for i in 0..4 {
                self.board[3 - i][column] = res[i];
            }
        }
    }

    pub fn move_left(&mut self) -> () {
        for row in 0..4 {
            let mut temp = Vec::new();
            let mut res: Vec<u32> = Vec::new();
            for column in 0..4 {
                if self.board[row][column] != 0 {
                    temp.push(self.board[row][column]);
                }
            }
            let mut index = 0;
            while index < temp.len() {
                if index + 1 < temp.len() && temp[index] == temp[index + 1] {
                    res.push(temp[index] * 2);
                    self.score += temp[index] * 2;
                    index += 2;
                } else {
                    res.push(temp[index]);
                    index += 1;
                }
            }
            while res.len() < 4 {
                res.push(0);
            }

            for i in 0..4 {
                self.board[row][i] = res[i];
            }
        }
    }

    pub fn move_right(&mut self) -> () {
        for row in 0..4 {
            let mut temp = Vec::new();
            let mut res: Vec<u32> = Vec::new();
            for column in (0..4).rev() {
                if self.board[row][column] != 0 {
                    temp.push(self.board[row][column]);
                }
            }
            let mut index = 0;
            while index < temp.len() {
                if index + 1 < temp.len() && temp[index] == temp[index + 1] {
                    res.push(temp[index] * 2);
                    self.score += temp[index] * 2;
                    index += 2;
                } else {
                    res.push(temp[index]);
                    index += 1;
                }
            }

            while res.len() < 4 {
                res.push(0);
            }

            for i in 0..4 {
                self.board[row][3 - i] = res[i];
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
        let mut rng = rand::rng();
        let empty_positions = self.get_empty_positions();
        let (row, column) = if !empty_positions.is_empty() {
            let &(row, column) = empty_positions
                .get(rng.random_range(0..empty_positions.len()))
                .unwrap();
            (row, column)
        } else {
            (4, 4) // Default value if no empty positions
        };
        if row == 4 && column == 4 {
            return;
        }
        let value = rng.random_range(1..=2) * 2; // 2或4
        self.board[row][column] = value;
    }

    pub fn run(&mut self) {
        'terminal: loop {
            println!("Starting a new game of 2048!");
            println!("Use arrow keys to move tiles. Press 'q' to quit.");
            self.reset();
            'game: loop {
                self.add_randon_element();
                // 使用 ANSI 转义符清屏
                print!("\x1B[2J\x1B[1;1H");
                
                self.display();
                if self.is_lose() {
                    println!("Game Over! Your score is: {}", self.score);
                    break 'game;
                }
                'inner: loop {
                    if let Some(key) = self.get_input() {
                        thread::sleep(Duration::from_millis(100));
                        match key {
                            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                                self.make_move(key);
                                break 'inner;
                            }
                            KeyCode::Char('q') => {
                                println!("Quitting the game. Your score is: {}", self.score);
                                break 'terminal;
                            }
                            _ => continue,
                        }
                    }
                }
            }
        }
    }
}