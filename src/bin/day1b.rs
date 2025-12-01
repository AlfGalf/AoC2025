use advent_of_code_2025::day1::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = import_data("inputs/day1")?;

    let (_, res): (isize, isize) = data.iter().fold((50, 0), |(pos, total), mov| {
        let new_pos = match mov.dir {
            Dir::Left => pos - mov.dist,
            Dir::Right => pos + mov.dist,
        };

        let new_total = if new_pos <= 0 {
            total - new_pos / 100 + if pos != 0 { 1 } else { 0 }
        } else {
            total + new_pos / 100
        };
        // println!("prev: {pos}, mov: {mov:?}, after: {new_pos} total: {total}");
        (new_pos.rem_euclid(100), new_total)
    });
    println!("{res}");
    Ok(())
}
