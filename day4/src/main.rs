mod models;

use itertools::{FoldWhile::*, Itertools};
use models::{Board, Cell, COLUMNS, ROWS};
use std::fs;
use std::io::Result;

static INPUT_PATH: &str = "test.txt";

fn main() -> Result<()> {
    let (nums, boards) = read_game(INPUT_PATH)?;
    Ok(())
}

fn read_game(filename: &str) -> Result<(Vec<u8>, Vec<Board>)> {
    let contents = fs::read_to_string(filename)?;
    let splits: Vec<&str> = contents.split("\n\n").collect();
    let nums_to_draw: Vec<u8> = splits[0]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    let boards: Vec<Board> = splits[1..].iter().map(|&row| Board::from(row)).collect();
    Ok((nums_to_draw, boards))
}
