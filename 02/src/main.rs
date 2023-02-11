use std::io::BufRead;

use regex::Regex;

fn compute_strat1(hand1: char, hand2: char) -> u16 {
    match (hand1, hand2) {
        ('A', 'X') => 1 + 3,
        ('A', 'Y') => 2 + 6,
        ('A', 'Z') => 3 + 0,
        ('B', 'X') => 1 + 0,
        ('B', 'Y') => 2 + 3,
        ('B', 'Z') => 3 + 6,
        ('C', 'X') => 1 + 6,
        ('C', 'Y') => 2 + 0,
        ('C', 'Z') => 3 + 3,
        _ => Err("Invalid hand").unwrap(),
    }
}

fn compute_strat2(hand1: char, hand2: char) -> u16 {
    match (hand1, hand2) {
        ('A', 'X') => 0 + 3,
        ('A', 'Y') => 3 + 1,
        ('A', 'Z') => 6 + 2,
        ('B', 'X') => 0 + 1,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,
        ('C', 'X') => 0 + 2,
        ('C', 'Y') => 3 + 3,
        ('C', 'Z') => 6 + 1,
        _ => Err("Invalid hand").unwrap(),
    }
}

fn main() -> () {
    let inputs: Vec<(char, char)> = std::io::stdin()
        .lock()
        .lines()
        .map(|line_result| line_result.unwrap())
        .map(|line| {
            let r = Regex::new(r"([ABC]) ([XYZ])").unwrap();
            let caps = r.captures(&line).unwrap();
            (
                caps.get(1).unwrap().as_str().chars().next().unwrap(),
                caps.get(2).unwrap().as_str().chars().next().unwrap(),
            )
        })
        .collect();
    let res = inputs
        .clone()
        .into_iter()
        .map(|(hand1, hand2)| compute_strat1(hand1, hand2))
        .reduce(|acc, x| acc + x)
        .unwrap();
    println!("Strat1 {}", res);
    let res = inputs
        .into_iter()
        .map(|(hand1, hand2)| compute_strat2(hand1, hand2))
        .reduce(|acc, x| acc + x)
        .unwrap();
    println!("Strat2 {}", res);
}
