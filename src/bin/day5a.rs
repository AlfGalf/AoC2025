use advent_of_code_2025::day5::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let (fresh_ranges, ingredients) = import_data("inputs/day5test")?;
    let (fresh_ranges, ingredients) = import_data("inputs/day5")?;

    let res = ingredients
        .iter()
        .filter(|i| fresh_ranges.iter().any(|r| r.contains(i)))
        .count();

    println!("{res}");
    Ok(())
}
