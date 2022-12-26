use anyhow::Result;

use std::fs::{self, read_to_string};
use std::time::SystemTime;

use crate::turtle::Command;
pub struct Script {
    path_to_watch: String,
    last_modified: SystemTime,
}

impl Script {
    pub fn new(path_to_watch: &str) -> Self {
        let last_modified =
            Self::read_metadata(path_to_watch).expect("First metadata reading failed!");
        Self {
            path_to_watch: path_to_watch.to_string(),
            last_modified,
        }
    }

    pub fn parse(&self) -> Vec<Command> {
        let mut commands = Vec::new();
        for (i, line) in read_to_string(&self.path_to_watch)
            .unwrap()
            .lines()
            .filter(|line| !(line.len() == 0))
            .enumerate()
        {
            let mut command = None;
            let mut argument = None;
            for (i, word) in line.split_ascii_whitespace().enumerate() {
                match i {
                    0 => match word {
                        "move" => command = Some(Command::Move(0)),
                        "turn" => command = Some(Command::Turn(0)),
                        _ => (),
                    },
                    1 => argument = Some((word.parse::<i32>()).unwrap()),
                    _ => (),
                }
            }
            if command.is_none() || argument.is_none() {
                panic!("Error parsing script on line {i}");
            }
            commands.push(match command.unwrap() {
                Command::Move(_) => Command::Move(argument.unwrap()),
                Command::Turn(_) => Command::Turn(argument.unwrap()),
            });
        }
        commands
    }

    pub fn update(&mut self) -> Option<Vec<Command>> {
        if let Ok(modified) = Self::read_metadata(&self.path_to_watch) {
            if modified != self.last_modified {
                self.last_modified = modified;
                return Some(self.parse());
            }
        }
        None
    }

    pub fn read_metadata(path: &str) -> Result<SystemTime> {
        let metadata = fs::metadata(path)?;
        assert!(metadata.is_file());
        // TODO return err instead of panicing.
        Ok(metadata.modified()?)
    }
}
