mod models;
use models::Board;
use std::fs;
use std::io::Result;

static INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    let (nums, boards) = read_game(INPUT_PATH)?;
    let winners = play_game(&nums, &boards);
    // First puzzle
    let (winning_num, winning_board) = winners.first().unwrap();
    println!("First puzzle: {}", winning_num * winning_board.get_score());
    // Second puzzle
    let (looser_num, looser_board) = winners.last().unwrap();
    println!("Second puzzle: {}", looser_num * looser_board.get_score());
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

fn play_game(nums: &[u16], boards: &[Board]) -> Vec<(u16, Board)> {
    let (winning_boards, _) = nums
        .iter()
        // Reduce nums list to a tuple of (winner_boards, remaining_boards)
        // On each iteration we add the new winners to the winner_boards list
        // and remove them from the remaining_boards list
        .fold((vec![], Vec::from(boards)), |(winners, remaining), &num| {
            let remaining = remaining.iter().map(|board| board.mark(num));
            let round_winners = remaining
                .clone()
                .filter(|board| board.has_won())
                .map(|board| (num, board));
            let non_winners = remaining.into_iter().filter(|board| !board.has_won());
            (
                winners.into_iter().chain(round_winners).collect(),
                non_winners.collect(),
            )
        });
    winning_boards
}
