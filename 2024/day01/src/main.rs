use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn load_input(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            left_list.push(parts[0].parse::<i32>().unwrap());
            right_list.push(parts[1].parse::<i32>().unwrap());
        }
    }

    Ok((left_list, right_list))
}

fn part_one() -> io::Result<()> {
    let (mut left_list, mut right_list) = load_input("./input.txt")?;

    left_list.sort();
    right_list.sort();

    let total_distance: i32 = left_list.iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("Part 1: {}", total_distance);

    Ok(())
}

fn part_two() -> io::Result<()> {
    let (left_list, right_list) = load_input("./input.txt")?;

    let right_count = right_list.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let total_distance: i32 = left_list.iter()
        .map(|&left| left * right_count.get(&left).unwrap_or(&0))
        .sum();

    println!("Part 2: {}", total_distance);

    Ok(())
}

fn main() -> io::Result<()> {
    part_one()?;
    part_two()?;

    Ok(())
}