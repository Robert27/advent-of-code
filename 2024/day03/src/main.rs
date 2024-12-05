use regex::Regex;


fn part01(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for cap in re.captures_iter(input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        sum += x * y;
    }

    sum
}

fn part02(input: &str) -> i32 {
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;

    let sum = re.captures_iter(input)
        .filter(|capture| {
            if capture.get(0).unwrap().as_str() == "do()" {
                enabled = true;
                return false;
            } else if capture.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
            }
            enabled
        })
        .map(|capture| {
            capture.get(2).unwrap().as_str().parse::<usize>().unwrap()
                * capture.get(3).unwrap().as_str().parse::<usize>().unwrap()
        })
        .sum::<usize>();
    sum as i32
}


fn main() {
    let input = include_str!("../input.txt");
    println!("Part 01: {}", part01(input));
    println!("Part 02: {}", part02(input));
}