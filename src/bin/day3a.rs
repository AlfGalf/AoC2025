use advent_of_code_2025::day3::*;

use std::error::Error;

fn get_max(vals: &Vec<u8>) -> u8 {
    // println!("vals {vals:?}");
    let (pos, digit1) = vals
        .iter()
        .enumerate()
        .take(vals.len() - 1)
        .rev()
        .max_by_key(|(_, d)| *d)
        .unwrap();

    let digit2 = vals.iter().skip(pos + 1).max().unwrap();

    // println!("d1: {digit1} d2: {digit2}");

    return digit1 * 10 + digit2;
}

fn main() -> Result<(), Box<dyn Error>> {
    // let data = import_data("inputs/day3test")?;
    let data = import_data("inputs/day3")?;

    let res: usize = data.into_iter().map(|arr| get_max(&arr) as usize).sum();
    println!("{res}");
    Ok(())
}
