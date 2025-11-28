// 通过crossterm获取键盘输入
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crate::model::Game2048;



impl Game2048 {
    pub fn get_input(&self) -> Option<KeyCode> {
        if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
            Some(code)
        } else {
            None
        }
    }
}