use glam::Vec2;
use minifb::{self, Key};

use crate::script::Script;
use crate::turtle::{Command, Turtle};

pub struct Window {
    turtle: Turtle,
    script: Script,
    width: usize,
    height: usize,
    elapsed_updates: u32,
    should_stop: bool,
    window: minifb::Window,
    buffer: Vec<u32>,
}

impl Window {
    pub fn new(width: usize, height: usize, script_path: &str) -> Self {
        let buffer: Vec<u32> = vec![0; width * height];

        let mut window =
            minifb::Window::new("FLOATING", width, height, minifb::WindowOptions::default())
                .unwrap_or_else(|e| {
                    panic!("{}", e);
                });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        Self {
            turtle: Turtle::new(Vec2::new((width / 2) as f32, (height / 2) as f32)),
            script: Script::new(script_path),
            elapsed_updates: 0,
            window,
            width,
            height,
            buffer,
            should_stop: false,
        }
    }

    pub fn run(&mut self) {
        let commands = vec![
            Command::Move(50),
            Command::Turn(90),
            Command::Move(50),
            Command::Turn(90),
        ];
        self.init(Some(&commands));

        while !self.should_stop && self.window.is_open() {
            self.update();
            self.draw();
        }
    }

    pub fn init(&mut self, commands: Option<&[Command]>) {
        if let Some(commands) = commands {
            self.turtle.apply_commands(commands);
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
        let points = self.turtle.draw();

        for point in points.iter().filter(|p| {
            p.x > 0.0 && p.x < self.width as f32 && p.y > 0.0 && p.y < self.height as f32
        }) {
            let x = point.x as i32;
            let y = (point.y as i32) * (self.width as i32);
            self.buffer[(x + y) as usize] = u32::MAX;
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
}
