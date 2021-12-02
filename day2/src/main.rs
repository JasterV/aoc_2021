mod models;

use models::{Command, Submarine, SubmarineV1, SubmarineV2};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::iter::Iterator;
use std::path::Path;

static INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    let submarine_v1 = SubmarineV1 { xcord: 0, depth: 0 };
    let submarine_v2 = SubmarineV2 {
        xcord: 0,
        depth: 0,
        aim: 0,
    };

    let commands = read_commands(INPUT_PATH)?;
    let submarine_v1 = apply_commands(submarine_v1, &commands);
    let submarine_v2 = apply_commands(submarine_v2, &commands);

    println!("First puzzle: {}", submarine_v1.position_x_depth());
    println!("Second puzzle: {}", submarine_v2.position_x_depth());
    Ok(())
}

fn apply_commands<T: Submarine>(submarine: T, commands: &[Command]) -> T {
    commands.iter().copied().fold(submarine, move_submarine)
}

fn move_submarine<T: Submarine>(submarine: T, command: Command) -> T {
    match command {
        Command::Forward(unit) => submarine.forward(unit),
        Command::Down(unit) => submarine.down(unit),
        Command::Up(unit) => submarine.up(unit),
    }
}

fn read_commands(file_path: &str) -> Result<Vec<Command>> {
    read_lines(file_path).map(lines_to_commands)
}

fn lines_to_commands(lines: Lines<BufReader<File>>) -> Vec<Command> {
    lines
        .filter_map(|line| line.ok().map(Command::from))
        .collect()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
