use std::fs;
use std::io::Result;
use std::iter::Iterator;

fn main() -> Result<()> {
    let input_path = "./input.txt";
    let positions = read_positions(input_path)?;
    let (optimal_position, fuel) = optimal_alignment_position(&positions);
    Ok(println!(
        "Best position to align to: {optimal_position}. Fuel: {fuel}"
    ))
}

fn optimal_alignment_position(positions: &[i32]) -> (i32, i32) {
    let &min = positions.iter().min().unwrap();
    let &max = positions.iter().max().unwrap();
    (min..=max)
        .map(|pos| {
            let fuel = calculate_alignment_fuel_cost(pos, positions);
            (pos, fuel)
        })
        .min_by_key(|&(_, fuel)| fuel)
        .unwrap()
}

fn calculate_alignment_fuel_cost(align_to: i32, positions: &[i32]) -> i32 {
    positions
        .into_iter()
        .map(|pos| i32::abs(align_to - pos))
        .sum()
}

fn read_positions(path: &str) -> Result<Vec<i32>> {
    let content = fs::read_to_string(path)?;
    let positions = content
        .trim()
        .split(',')
        .filter_map(|elem| elem.parse::<i32>().ok())
        .collect();
    Ok(positions)
}
