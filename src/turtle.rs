use glam::Vec2;

use crate::geometry::{Line, Triangle};

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Move(i32),
    Turn(i32),
}

#[derive(Debug)]
pub struct Turtle {
    starting_position: Vec2,
    position: Vec2,
    rotation: i32,
    path: Vec<Line>,
}

impl Turtle {
    pub const fn new(starting_position: Vec2) -> Self {
        Self {
            starting_position,
            position: starting_position,
            rotation: 0,
            path: Vec::new(),
        }
    }

    fn forward(&self) -> Vec2 {
        Vec2::from_angle((self.rotation as f32).to_radians())
    }

    pub fn reset(&mut self) {
        self.path = Vec::new();
        self.position = self.starting_position;
        self.rotation = 0;
    }

    pub fn apply_command(&mut self, command: Command) {
        match command {
            Command::Move(forward_distance) => {
                let start = self.position;
                println!("Turtle moved {forward_distance}");
                self.position += self.forward() * (forward_distance as f32);
                let end = self.position;
                self.path.push(Line::new(start, end));
            }
            Command::Turn(clockwise_degrees) => {
                println!("Turtle turned {clockwise_degrees}");
                self.rotation += clockwise_degrees;
                self.rotation %= 360;
            }
        }
        println!("New turtle state: {self:?}");
    }

    pub fn apply_commands(&mut self, commands: &[Command]) {
        for command in commands {
            self.apply_command(*command)
        }
    }

    pub fn turtle_triangle(&self) -> Triangle {
        let triangle_size = 5.0;
        let heading = self.forward() * triangle_size;
        let side1 = Vec2::from_angle((90.0f32).to_radians()).rotate(self.forward()) * triangle_size;
        let side2 =
            Vec2::from_angle((270.0f32).to_radians()).rotate(self.forward()) * triangle_size;
        Triangle::new(
            self.position + heading,
            self.position + side1,
            self.position + side2,
        )
    }

    pub fn draw(&self) -> Vec<Vec2> {
        let mut points = Vec::new();
        for line in &self.path {
            points.extend(line.solid().iter())
        }
        points.extend(self.turtle_triangle().solid_color());
        points
    }
}
