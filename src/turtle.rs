use glam::Vec2;

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Move(i32),
    Turn(i32),
}

#[derive(Debug)]
pub struct Turtle {
    position: Vec2,
    rotation: i32,
}

impl Turtle {
    pub const fn new(position: Vec2) -> Self {
        Self {
            position,
            rotation: 0,
        }
    }

    pub fn apply_command(&mut self, command: Command) {
        match command {
            Command::Move(forward_distance) => {
                println!("Turtle moved {forward_distance}");
                let forward = Vec2::from_angle((self.rotation as f32).to_radians())
                    * (forward_distance as f32);
                self.position += forward;
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
}
