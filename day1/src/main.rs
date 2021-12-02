use std::fs::File;
use std::io::{self, BufRead};
use std::io::{BufReader, Lines, Result};
use std::iter::Iterator;
use std::path::Path;

const INPUT_PATH: &'static str = "./input.txt";

fn main() -> Result<()> {
    let depths = read_depths(INPUT_PATH)?;
    // First puzzle
    println!("First puzzle {0}", count_increases(&depths));
    // Second puzzle
    let sliding_window_sums = calculate_sliding_window_sums(&depths, 3);
    println!("Second puzzle {0}", count_increases(&sliding_window_sums));
    Ok(())
}

fn calculate_sliding_window_sums(depths: &[i32], size: usize) -> Vec<i32> {
    (0..=depths.len() - size)
        .map(|i| &depths[i..i + size])
        .map(|slice| slice.iter().sum())
        .collect()
}

fn count_increases(nums: &[i32]) -> i32 {
    // Map an iterator of nums to [last_num, counter]
    let [_, counter] = nums.iter().fold([0, -1], |[prev, counter], &curr| {
        [curr, counter + if curr > prev { 1 } else { 0 }]
    });
    counter
}

fn read_depths(file_path: &str) -> Result<Vec<i32>> {
    read_lines(file_path).map(lines_to_depths)
}

fn lines_to_depths(lines: Lines<BufReader<File>>) -> Vec<i32> {
    lines
        .filter_map(|line| line.ok().map(line_to_depth))
        .collect()
}

fn line_to_depth(line: String) -> i32 {
    line.parse::<i32>().unwrap()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
