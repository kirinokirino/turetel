#![warn(clippy::nursery)]

mod script;
mod turtle;
mod window;

use crate::window::Window;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const PATH: &str = "./main.turtle";

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, PATH);

    window.run();
}
