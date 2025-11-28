// 通过crossterm获取键盘输入
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crate::model::Game2048;

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
        }
    }

    pub fn move_down(&mut self) -> () {

    }

    pub fn move_left(&mut self) -> () {

    }

    pub fn move_right(&mut self) -> () {

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
}