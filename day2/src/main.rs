mod models;

use models::{Command, Submarine, SubmarineV1, SubmarineV2};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::iter::Iterator;
use std::path::Path;

static INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    // First puzzle
    let submarine_v1 = SubmarineV1::new();
    let commands = read_commands(INPUT_PATH)?;
    let submarine_v1 = apply_commands(submarine_v1, &commands);
    println!("First puzzle: {}", submarine_v1.position_x_depth());
    // Second puzzle
    let submarine_v2 = SubmarineV2::new();
    let submarine_v2 = apply_commands(submarine_v2, &commands);
    println!("Second puzzle: {}", submarine_v2.position_x_depth());
    Ok(())
}

fn apply_commands(submarine: impl Submarine, commands: &[Command]) -> impl Submarine {
    commands
        .iter()
        .copied()
        .fold(submarine, |submarine, command| submarine.exec(command))
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
