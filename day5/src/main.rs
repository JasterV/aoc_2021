mod line;
use line::Line;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};
static INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    let overlapping_points_count = read_input(INPUT_PATH)?
        .flat_map(|line| line.get_points())
        .fold(HashMap::new(), |mut counters, point| {
            counters.insert(point, counters.get(&point).map_or(1, |count| count + 1));
            counters
        })
        .into_values()
        .filter(|&count| count >= 2)
        .count();
    println!("Overlapping points: {}", overlapping_points_count);
    Ok(())
}

fn read_input(filename: &str) -> Result<impl Iterator<Item = Line>> {
    Ok(read_lines(filename)?.filter_map(|line| line.ok().map(Line::from)))
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
