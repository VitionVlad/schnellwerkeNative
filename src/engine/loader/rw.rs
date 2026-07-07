use std::fs;

pub fn readfs(path: &str) -> Vec<u8>{
    fs::read(path).unwrap()
}