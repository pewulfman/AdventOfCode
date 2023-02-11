use std::{
    collections::HashSet,
    io::{stdin, BufRead},
};

fn priority(item_type: char) -> u32 {
    match item_type {
        'A'..='Z' => (item_type as u32) - ('A' as u32) + 27,
        _ => (item_type as u32) - ('a' as u32) + 1,
    }
}

fn find_missplaced(line: String) -> char {
    let size = line.len();
    let (pocket1, pocket2) = line.split_at(size / 2);
    let pocket1_items: HashSet<char> = pocket1.chars().collect();
    let common = pocket2.chars().filter(|c| pocket1_items.contains(&c));
    common.into_iter().next().unwrap()
}

fn find_common(group: &[String]) -> char {
    group
        .iter()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .reduce(|a, b| a.intersection(&b).cloned().collect())
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

fn main() {
    let rucksacks: Vec<String> = stdin().lock().lines().map(|line| line.unwrap()).collect();
    //let mut res = 0;
    let res = rucksacks
        .iter()
        .map(|line| find_missplaced(line.to_string()))
        .map(|c| priority(c))
        .sum::<u32>();
    println!("res : {}", res);

    let res2 = rucksacks
        .chunks(3)
        .map(|chunk| find_common(chunk))
        .map(priority)
        .sum::<u32>();
    println!("res2 : {}", res2);
}
