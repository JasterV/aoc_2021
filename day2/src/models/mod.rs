#[derive(Debug, Clone, Copy)]
pub struct Submarine {
    pub xcord: i32,
    pub depth: i32,
}

impl Submarine {
    pub fn exec(submarine: Self, command: Command) -> Self {
        match command {
            Command::Forward(unit) => Self::forward(submarine, unit),
            Command::Down(unit) => Self::down(submarine, unit),
            Command::Up(unit) => Self::up(submarine, unit),
        }
    }

    fn forward(submarine: Self, unit: i32) -> Self {
        Submarine {
            xcord: submarine.xcord + unit,
            depth: submarine.depth,
        }
    }

    fn down(submarine: Self, unit: i32) -> Self {
        Submarine {
            xcord: submarine.xcord,
            depth: submarine.depth + unit,
        }
    }

    fn up(submarine: Self, unit: i32) -> Self {
        Submarine {
            xcord: submarine.xcord,
            depth: submarine.depth - unit,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl From<String> for Command {
    fn from(str: String) -> Self {
        let words: Vec<&str> = str.split_ascii_whitespace().collect();
        let action = words[0];
        let unit: i32 = words[1].parse().unwrap();
        match action {
            "forward" => Command::Forward(unit),
            "down" => Command::Down(unit),
            "up" => Command::Up(unit),
            _ => panic!("Invalid command {}", action),
        }
    }
}
