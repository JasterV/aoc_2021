use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

static INPUT_PATH: &str = "input.txt";

fn main() -> Result<()> {
    let diagnostic_report = read_diagnostic_report(INPUT_PATH)?;
    let [gamma_rate, epsilon_rate] = extract_report_rates(&diagnostic_report);
    println!("First puzzle: {}", gamma_rate * epsilon_rate);
    Ok(())
}

fn extract_report_rates(report: &[String]) -> [i32; 2] {
    let sums = compute_digits_sums(report);
    let gamma_rate: String = sums
        .iter()
        .map(|&sum| if sum > 0 { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");
    let epsilon_rate: String = sums
        .iter()
        .map(|&sum| if sum > 0 { "0" } else { "1" })
        .collect::<Vec<&str>>()
        .join("");
    [
        binary_str_to_int(&gamma_rate),
        binary_str_to_int(&epsilon_rate),
    ]
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
