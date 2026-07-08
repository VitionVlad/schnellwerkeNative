#![allow(dead_code)]

use std::fs;

pub fn readfs(path: &str) -> Vec<u8>{
    fs::read(path).unwrap()
}

pub fn checkfs(path: &str) -> bool{
    fs::exists(path).unwrap()
}

pub fn writefs(path: &str, content: Vec<u8>){
    let _ = fs::write(path, content);
}