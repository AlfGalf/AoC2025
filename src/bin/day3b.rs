use advent_of_code_2025::day3::*;

use std::error::Error;

fn get_max(vals: &Vec<u8>) -> usize {
    let mut res: usize = 0;

    // dbg!(vals);

    let mut skips = 0;
    for i in 0..12 {
        res *= 10;

        let (pos, digit) = vals
            .iter()
            .enumerate()
            .take(vals.len() - 11 + i)
            .skip(skips)
            .rev()
            .max_by_key(|(_, d)| *d)
            .unwrap();

        res += *digit as usize;
        skips = pos + 1;
    }

    // dbg!(res)
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    // let data = import_data("inputs/day3test")?;
    let data = import_data("inputs/day3")?;

    let res: usize = data.into_iter().map(|arr| get_max(&arr) as usize).sum();
    println!("{res}");
    Ok(())
}
