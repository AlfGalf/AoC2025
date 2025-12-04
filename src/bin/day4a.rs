use advent_of_code_2025::day4::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let data = import_data("inputs/day4test")?;
    let data = import_data("inputs/day4")?;

    let width = data.len();
    let height = data[0].len();

    let mut adjacents = vec![vec![0; height]; width];

    for x in 0..width {
        for y in 0..height {
            if data[x as usize][y as usize] == '@' as u8 {
                for (nx, ny) in get_neighbours((x, y), (width, height)) {
                    adjacents[nx as usize][ny as usize] += 1;
                }
            }
        }
    }

    let res: usize = data
        .iter()
        .zip(adjacents.iter())
        .map(|(dv, av)| {
            dv.iter()
                .zip(av.iter())
                .filter(|(d, a)| **d == b'@' && **a < 4)
                .count()
        })
        .sum();

    println!("{res}");
    Ok(())
}
