use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

static INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    let diagnostic_report = read_diagnostic_report(INPUT_PATH)?;
    let [gamma_rate, epsilon_rate, oxygen_rate, co2_rate] =
        extract_report_rates(&diagnostic_report);
    println!("First puzzle: {}", gamma_rate * epsilon_rate);
    println!("Second puzzle {}", oxygen_rate * co2_rate);
    Ok(())
}

fn extract_report_rates(report: &[String]) -> [i32; 4] {
    let sums = compute_digits_sums(report);
    let gamma_rate = map_sums(&sums, |&sum| if sum > 0 { "1" } else { "0" }).join("");
    let epsilon_rate = map_sums(&sums, |&sum| if sum > 0 { "0" } else { "1" }).join("");
    let oxygen_rate = filter_report_while(
        report,
        |report, iteration| {
            filter_report_by_nums_bit(report, iteration, |bit_sums, curr_bit| {
                bit_sums >= 0 && curr_bit == '1' || bit_sums < 0 && curr_bit == '0'
            })
        },
        |report, iteration| report.len() == 1 || iteration > sums.len(),
    )
    .first()
    .unwrap()
    .clone();

    let co2_rate = filter_report_while(
        report,
        |report, bit_index| {
            filter_report_by_nums_bit(report, bit_index, |bit_sums, curr_bit| {
                bit_sums >= 0 && curr_bit == '0' || bit_sums < 0 && curr_bit == '1'
            })
        },
        |report, iteration| report.len() == 1 || iteration > sums.len(),
    )
    .first()
    .unwrap()
    .clone();
    [
        binary_str_to_int(&gamma_rate),
        binary_str_to_int(&epsilon_rate),
        binary_str_to_int(&oxygen_rate),
        binary_str_to_int(&co2_rate),
    ]
}

fn filter_report_by_nums_bit<T>(report: Vec<String>, bit_index: usize, mut f: T) -> Vec<String>
where
    T: FnMut(i32, char) -> bool,
{
    let sum = compute_digits_sums(&report)[bit_index];
    report
        .into_iter()
        .filter(|num| {
            let c = get_char_at_index(num, bit_index);
            f(sum, c)
        })
        .collect()
}

fn filter_report_while<T, H>(report: &[String], mut f: T, mut w: H) -> Vec<String>
where
    T: FnMut(Vec<String>, usize) -> Vec<String>,
    H: FnMut(&[String], usize) -> bool,
{
    (0..)
        .fold_while(Vec::from(report), |report, index| {
            if w(&report, index) {
                return Done(report);
            }
            Continue(f(report, index))
        })
        .into_inner()
}

fn get_char_at_index(word: &str, index: usize) -> char {
    word[index..].chars().next().unwrap()
}

fn map_sums<T, F>(sums: &[i32], f: F) -> Vec<T>
where
    F: FnMut(&i32) -> T,
{
    sums.iter().map(f).collect::<Vec<T>>()
}

fn compute_digits_sums(report: &[String]) -> Vec<i32> {
    report.iter().fold(vec![], |sums, curr| {
        curr.chars()
            .enumerate()
            .map(|(index, char)| {
                let value = if char == '0' { -1 } else { 1 };
                sums.get(index).map_or(value, |sum| sum + value)
            })
            .collect()
    })
}

fn read_diagnostic_report(file_path: &str) -> Result<Vec<String>> {
    read_lines(file_path).map(|lines| lines.filter_map(|line| line.ok()).collect())
}

fn binary_str_to_int(binary_str: &str) -> i32 {
    i32::from_str_radix(binary_str, 2).unwrap()
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
