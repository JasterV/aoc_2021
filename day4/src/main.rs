mod models;

use itertools::{FoldWhile::*, Itertools};
use models::Board;
use std::fs;
use std::io::Result;

static INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    let (nums, boards) = read_game(INPUT_PATH)?;
    let (last_num, winning_board) = play_game(&nums, &boards);
    let unmarked_cells_sum = winning_board
        .iter()
        .filter(|cell| !cell.is_marked())
        .map(|cell| cell.into_inner())
        .sum::<u16>();
    println!("First puzzle: {}", last_num * unmarked_cells_sum);
    Ok(())
}

fn read_game(filename: &str) -> Result<(Vec<u16>, Vec<Board>)> {
    let contents = fs::read_to_string(filename)?;
    let splits: Vec<&str> = contents.split("\n\n").collect();
    let nums_to_draw: Vec<u16> = splits[0]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    let boards: Vec<Board> = splits[1..].iter().map(|&row| Board::from(row)).collect();
    Ok((nums_to_draw, boards))
}

fn play_game(nums: &[u16], boards: &[Board]) -> (u16, Board) {
    let (last_num, boards) = nums
        .iter()
        .fold_while((0, Vec::from(boards)), |(last_num, boards), &num| {
            if boards.iter().any(|board| board.has_won()) {
                return Done((last_num, boards));
            }
            Continue((num, boards.iter().map(|board| board.mark(num)).collect()))
        })
        .into_inner();
    let winning_board = boards.into_iter().find(|board| board.has_won()).unwrap();
    (last_num, winning_board)
}
