use advent_of_code_2025::day2::*;

use std::error::Error;

fn is_valid(val: &usize) -> bool {
    let str: String = val.to_string();
    if str.len() % 2 != 0 {
        return true;
    }
    let len = str.len();

    !(str[..len / 2] == str[len / 2..])
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data("inputs/day2")?;

    let res: usize = data.into_iter().fold(0, |total: usize, range| {
        total + range.filter(|i| !is_valid(i)).sum::<usize>()
    });
    println!("{res}");
    Ok(())
}
