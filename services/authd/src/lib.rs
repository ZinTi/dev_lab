// src/lib.rs

// 1. pub mod 允许外部代码使用以下模块
pub mod config; // src/config/mod.rs
pub mod dao; // src/dao/mod.rs
pub mod ffi; // src/ffi/mod.rs

// 2. use 起别名，pub use 导出别名

use std::io::{self, Write};

use crate::{config::config::PromptConf};

pub struct App {
    conf: config::config::Config,

}

impl App {
    pub fn new() -> Self {
        let conf = config::config::Config::load().expect("Failed to load config");

        let prompt = conf.prompt.clone();

        App {
            conf: conf,


        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn run(&mut self){
    config::about::about::print_about();
    println!("{}\n", self.hint);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
