use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn load_input(file_path: &str) -> Vec<Vec<i32>> {
    let path = Path::new(file_path);
    let file = File::open(&path).expect("Could not open file");
    let reader = io::BufReader::new(file);

    reader.lines()
        .map(|line| {
            line.expect("Could not read line")
                .split_whitespace()
                .map(|num| num.parse().expect("Could not parse number"))
                .collect()
        })
        .collect()
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff > 0 {
            decreasing = false;
        } else if diff < 0 {
            increasing = false;
        }
    }

    increasing || decreasing
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    if is_safe_report(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_safe_report(&modified_report) {
            return true;
        }
    }

    false
}

fn part_one() {
    let file_path = "input.txt";
    let input = load_input(file_path);
    let safe_reports_count = input.iter().filter(|report| is_safe_report(report)).count();
    println!("Number of safe reports: {}", safe_reports_count);
}

fn part_two() {
    let file_path = "input.txt";
    let input = load_input(file_path);
    let safe_reports_count = input.iter().filter(|report| is_safe_with_dampener(report)).count();
    println!("Number of safe reports with dampener: {}", safe_reports_count);
}

fn main() {
    part_one();
    part_two();
}