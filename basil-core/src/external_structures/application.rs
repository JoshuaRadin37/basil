use std::process::exit;
use std::cell::Cell;

pub struct Application {
    run: bool,
    exit_code: i32
}

impl Application {

    pub fn run(mut self) -> Self {
        self.run = true;
        loop {


            if !self.run {
                break;
            }
        }
        self
    }

    pub fn quit(&mut self, exit_code: i32) {
        self.run = false;
        self.exit_code = exit_code;
    }
}