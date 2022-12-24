#[warn(clippy::nursery)]
use minifb::{self, Key};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut window = Window::new();

    window.run();
}

struct Window {
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
        Self { window, buffer, should_stop: false }
    }

    pub fn run(&mut self) {
        while !self.should_stop && self.window.is_open() {
            self.update();
            self.draw();
        }
    }

    pub fn update(&mut self) {
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
