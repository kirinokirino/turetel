use glam::Vec2;

use crate::geometry::Line;

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Move(i32),
    Turn(i32),
}

#[derive(Debug)]
pub struct Turtle {
    position: Vec2,
    rotation: i32,
    path: Vec<Line>,
}

impl Turtle {
    pub const fn new(position: Vec2) -> Self {
        Self {
            position,
            rotation: 0,
            path: Vec::new(),
        }
    }

    pub fn apply_command(&mut self, command: Command) {
        match command {
            Command::Move(forward_distance) => {
                let start = self.position;
                println!("Turtle moved {forward_distance}");
                let forward = Vec2::from_angle((self.rotation as f32).to_radians())
                    * (forward_distance as f32);
                self.position += forward;
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

    pub fn draw(&self) -> Vec<Vec2> {
        let mut points = Vec::new();
        for line in &self.path {
            points.extend(line.solid().iter())
        }
        // TODO : Draw turtle
        points
    }
}
