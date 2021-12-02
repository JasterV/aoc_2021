mod models;

use models::{Command, Submarine};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::iter::Iterator;
use std::path::Path;

static INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    let submarine = Submarine { xcord: 0, depth: 0 };
    let commands = read_commands(INPUT_PATH)?;
    let submarine = apply_commands(submarine, &commands);
    println!("First puzzle: {}", submarine.depth * submarine.xcord);
    Ok(())
}

fn apply_commands(submarine: Submarine, commands: &[Command]) -> Submarine {
    commands.iter().copied().fold(submarine, Submarine::exec)
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
