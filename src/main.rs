#![warn(clippy::nursery)]
use anyhow::Result;
use minifb::{self, Key};

use std::fs;
use std::thread::sleep;
use std::time::{SystemTime};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const PATH: &str = "./main.turtle";

fn main() {
    let mut window = Window::new();

    window.run();
}

struct Script {
    last_modified: SystemTime,
}

impl Script {
    pub fn new() -> Self {
        let last_modified = Script::read_metadata().expect("First metadata reading failed!");
        Self { last_modified }
    }

    pub fn update(&mut self) -> bool {
        if let Ok(modified) = Script::read_metadata() {
            if modified != self.last_modified {
                self.last_modified = modified;
                return true;
            }
        }
        false
    }

    pub fn read_metadata() -> Result<SystemTime> {
        let metadata = fs::metadata(PATH)?;
        assert!(metadata.is_file());
        // TODO return err instead of panicing.
        Ok(metadata.modified()?)
    }
}

struct Window {
    script: Script,
    elapsed_updates: u32,
    should_stop: bool,
    window: minifb::Window,
    buffer: Vec<u32>,
}

impl Window {
    pub fn new() -> Self {
        let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        let mut window =
            minifb::Window::new("FLOATING", WIDTH, HEIGHT, minifb::WindowOptions::default())
                .unwrap_or_else(|e| {
                    panic!("{}", e);
                });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        Self {
            script: Script::new(),
            elapsed_updates: 0,
            window,
            buffer,
            should_stop: false,
        }
    }

    pub fn run(&mut self) {
        while !self.should_stop && self.window.is_open() {
            self.update();
            self.draw();
        }
    }

    pub fn update(&mut self) {
        if self.elapsed_updates % 120 == 0 {
            if self.script.update() {
                println!("Updated script!");
            }
        }
        if self.window.is_key_down(Key::Escape) {
            self.should_stop = true;
        }
    }

    pub fn draw(&mut self) {
        for i in self.buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
