use glam::Vec2;

use crate::geometry::{Line, Triangle};

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, PartialEq)]
pub struct Scope(pub usize);

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Move(Scope, i32),
    Turn(Scope, i32),
    Repeat(Scope, i32),
}

impl Command {
    pub fn scope(&self) -> Scope {
        match self {
            Command::Move(scope, _) => return *scope,
            Command::Turn(scope, _) => return *scope,
            Command::Repeat(scope, _) => return *scope,
        }
    }
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
            Command::Move(_, forward_distance) => {
                let start = self.position;
                println!("Turtle moved {forward_distance}");
                self.position += self.forward() * (forward_distance as f32);
                let end = self.position;
                self.path.push(Line::new(start, end));
            }
            Command::Turn(_, clockwise_degrees) => {
                println!("Turtle turned {clockwise_degrees}");
                self.rotation += clockwise_degrees;
                self.rotation %= 360;
            }
            Command::Repeat(..) => panic!("Repeat makes sense only with other commands."),
        }
    }

    pub fn apply_commands(&mut self, commands: &[Command]) {
        let mut local_scope = Scope(0);
        let mut local_commands = Vec::new();
        for command in commands {
            match command {
                Command::Repeat(scope, times) => {
                    println!("Repeating {local_commands:?} {times} times");
                    for i in 0..*times {
                        self.apply_commands(&local_commands)
                    }
                }
                _ => self.apply_command(*command),
            }
            if (command.scope() == local_scope) {
                local_commands.push(*command)
            } else {
                local_commands = vec![*command];
                local_scope = command.scope();
            }
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
