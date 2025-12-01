use advent_of_code_2025::day1::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data("inputs/day1")?;

    let (_, res): (isize, isize) = data.iter().fold((50, 0), |(pos, total), mov| {
        let new_pos = match mov.dir {
            Dir::Left => pos - mov.dist,
            Dir::Right => pos + mov.dist,
        }
        .rem_euclid(100);
        // println!("prev: {pos}, mov: {mov:?}, after: {new_pos}");
        (new_pos, if new_pos == 0 { total + 1 } else { total })
    });
    println!("{res}");
    Ok(())
}
