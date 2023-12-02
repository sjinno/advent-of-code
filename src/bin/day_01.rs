use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use anyhow::Result;

const NUMBERS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() -> Result<()> {
    println!("eg 1: {}", do_the_thing("input/day_01_e1.input")?);
    println!("eg 2: {}", do_the_thing2("input/day_01_e2.input")?);
    println!("part 1: {}", do_the_thing("input/day_01.input")?);
    println!("part 2: {}", do_the_thing2("input/day_01.input")?);
    Ok(())
}

fn read_file_line_by_line(file_path: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

fn do_the_thing(file_path: &str) -> Result<u32> {
    let lines = read_file_line_by_line(file_path)?;
    Ok(lines
        .flatten()
        .filter_map(|line| {
            let nums = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            match (nums.first(), nums.last()) {
                (Some(first), Some(last)) => Some(first * 10 + last),
                (Some(first), None) => Some(first * 10 + first),
                _ => None,
            }
        })
        .sum())
}

fn do_the_thing2(file_path: &str) -> Result<u32> {
    let lines = read_file_line_by_line(file_path)?;
    let mut occurence_order = Vec::new();
    let mut sum = 0;

    for line in lines {
        let line = line?;

        NUMBERS.iter().for_each(|(word, num)| {
            occurence_order.extend(line.match_indices(word).map(|x| (x.0, word, num)));
        });

        occurence_order.sort_by(|a, b| a.0.cmp(&b.0));

        let first = occurence_order
            .iter()
            .fold(line.clone(), |acc, (_, &word, num)| {
                acc.replace(word, num.to_string().as_str())
            });
        let last = occurence_order
            .iter()
            .rev()
            .fold(line, |acc, (_, &word, num)| {
                acc.replace(word, num.to_string().as_str())
            });

        let h = first
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();
        let t = last
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();

        if let (Some(first), Some(last)) = (h.first(), t.last()) {
            sum += first * 10 + last;
        }

        occurence_order.clear();
    }

    Ok(sum)
}
