pub mod day1 {
    use std::{error::Error, fs::read_to_string};

    #[derive(Debug, Clone, Copy)]
    pub enum Dir {
        Left,
        Right,
    }
    #[derive(Debug, Clone, Copy)]
    pub struct InputLineData {
        pub dir: Dir,
        pub dist: isize,
    }

    pub fn import_data(file: &str) -> Result<Vec<InputLineData>, Box<dyn Error>> {
        let contents = read_to_string(file)?;
        Ok(contents
            .lines()
            .into_iter()
            .map(|line| InputLineData {
                dir: match line.chars().next().unwrap() {
                    'L' => Dir::Left,
                    'R' => Dir::Right,
                    _ => panic!(),
                },
                dist: line[1..].parse().unwrap(),
            })
            .collect())
    }
}

pub mod day2 {
    use std::{error::Error, fs::read_to_string, ops::RangeInclusive};

    pub fn import_data(file: &str) -> Result<Vec<RangeInclusive<usize>>, Box<dyn Error>> {
        let contents = read_to_string(file)?;
        Ok(contents
            .split(',')
            .into_iter()
            .map(|line| {
                let mut split = line.split('-');
                let (l1, l2) = (split.next().unwrap().trim(), split.next().unwrap().trim());
                // println!("'{l1}', '{l2}'");
                let low: usize = l1.parse().unwrap();
                let high: usize = l2.parse().unwrap();
                low..=high
            })
            .collect())
    }
}

pub mod day3 {
    use std::{error::Error, fs::read_to_string};

    pub fn import_data(file: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
        let contents = read_to_string(file)?;
        Ok(contents
            .lines()
            .into_iter()
            .map(|line| line.chars().map(|c| (c as u8 - '0' as u8)).collect())
            .collect())
    }
}

pub mod day4 {
    use std::{error::Error, fs::read_to_string};

    const NEIGHBOURS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    pub fn get_neighbours(
        (x, y): (usize, usize),
        (width, height): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        NEIGHBOURS.into_iter().filter_map(move |(dx, dy)| {
            let (nx, ny): (isize, isize) = (x as isize + dx, y as isize + dy);
            if nx < 0 || nx >= width as isize || ny < 0 || ny >= height as isize {
                None
            } else {
                Some((nx as usize, ny as usize))
            }
        })
    }

    pub fn import_data(file: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
        let contents = read_to_string(file)?;
        Ok(contents
            .lines()
            .into_iter()
            .map(|line| line.bytes().collect::<Vec<u8>>())
            .collect())
    }
}
