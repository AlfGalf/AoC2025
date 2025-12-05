use advent_of_code_2025::day5::*;

use std::{error::Error, ops::RangeInclusive};

fn main() -> Result<(), Box<dyn Error>> {
    // let (mut fresh_ranges, _) = import_data("inputs/day5test")?;
    let (mut fresh_ranges, _) = import_data("inputs/day5")?;

    let mut ranges: Vec<RangeInclusive<usize>> = Default::default();

    fresh_ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut fresh_rangers_iter = fresh_ranges.into_iter().peekable();

    while let Some(_range) = fresh_rangers_iter.next() {
        let mut range = _range.clone();

        while let Some(next) = fresh_rangers_iter.peek() {
            if next.start() > range.end() {
                break;
            }
            let next = fresh_rangers_iter.next().unwrap();
            range = *range.start()..=*range.end().max(next.end());
        }

        ranges.push(range);
    }

    dbg!(&ranges);

    let res: usize = ranges.into_iter().map(|r| r.count()).sum();
    println!("{res}");
    Ok(())
}
