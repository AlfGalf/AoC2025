use advent_of_code_2025::day4::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut data = import_data("inputs/day4test")?;
    let mut data = import_data("inputs/day4")?;

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

    while {
        let mut keep_going = false;

        for x in 0..width {
            for y in 0..height {
                if adjacents[x][y] < 4 && data[x][y] == '@' as u8 {
                    for (nx, ny) in get_neighbours((x, y), (height, width)) {
                        adjacents[nx][ny] -= 1;
                    }
                    data[x][y] = 'x' as u8;
                    keep_going = true;
                }
            }
        }

        keep_going
    } {}

    for x in 0..width {
        for y in 0..height {
            print!("{}", data[x][y] as char);
        }
        println!();
    }

    let res: usize = data
        .iter()
        .map(|dv| dv.iter().filter(|d| **d == b'x').count())
        .sum();

    println!("{res}");
    Ok(())
}
