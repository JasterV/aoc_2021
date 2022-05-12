mod models;

use models::{DigitDisplay, NoteEntry};
use std::{
    fs::File,
    io::Result,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let filepath = "./input.txt";
    let entries = read_note_entries(filepath)?;
    let sum: usize = entries
        .map(|entry| {
            entry
                .output
                .into_iter()
                .filter(is_unique_segments_digit)
                .count()
        })
        .sum();
    println!("There are {sum} unique digits");
    Ok(())
}

fn is_unique_segments_digit(display: &DigitDisplay) -> bool {
    match display.len() {
        2 | 3 | 4 | 7 => true,
        _ => false,
    }
}

fn read_note_entries(filepath: &str) -> Result<impl Iterator<Item = NoteEntry>> {
    let entries_iter = read_file_buf(filepath)?
        .lines()
        .map(|line| line.expect("Error reading file, it might be corrupted"))
        .map(NoteEntry::from);
    Ok(entries_iter)
}

fn read_file_buf(filepath: &str) -> Result<BufReader<File>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
