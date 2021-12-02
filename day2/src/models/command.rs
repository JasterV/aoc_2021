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
