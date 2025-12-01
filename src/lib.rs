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
