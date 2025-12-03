use advent_of_code_2025::day2::*;

use std::error::Error;

fn is_valid(val: &usize) -> bool {
    let len: usize = (*val as f64).log10().ceil() as usize;

    // println!("Val: {val} len: {len}");
    for i in 1..=len / 2 {
        if len % i != 0 {
            continue;
        }
        let recip = len / i;

        let quota = val % (10_usize.pow(i as u32));

        let guess: usize = (0..recip)
            .map(|v| 10_usize.pow((v * i) as u32) * quota as usize)
            .sum();

        // println!("i {i}, quota: {quota}, guess: {guess}");

        if &guess == val {
            // println!("Val: {val} len: {len}");
            // println!("FAKE");
            return false;
        }
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data("inputs/day2")?;

    let res: usize = data.into_iter().fold(0, |total: usize, range| {
        total + range.filter(|i| !is_valid(i)).sum::<usize>()
    });
    println!("{res}");
    Ok(())
}
