use std::io::{stdin, BufRead};

fn parse_section(section: String) -> ((u32, u32), (u32, u32)) {
    let (elf1, elf2) = section.split_once(",").unwrap();
    let (x1, y1) = elf1.split_once("-").unwrap();
    let (x2, y2) = elf2.split_once("-").unwrap();
    (
        (x1.parse().unwrap(), y1.parse().unwrap()),
        (x2.parse().unwrap(), y2.parse().unwrap()),
    )
}

fn total_overlap(&(x1, y1): &(u32, u32), &(x2, y2): &(u32, u32)) -> bool {
    x1 <= x2 && y2 <= y1 || x2 <= x1 && y1 <= y2
}

fn simple_overlap(&(x1, y1): &(u32, u32), &(x2, y2): &(u32, u32)) -> bool {
    x1 <= x2 && x2 <= y1 || x2 <= x1 && x1 <= y2
}

fn main() {
    println!("Finding Overlaps !");
    let sections: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(parse_section)
        .collect();

    let res = sections
        .iter()
        .filter(|(p1, p2)| total_overlap(p1, p2))
        .count();
    println!("Numbr of total overlaps: {}", res);

    let res = sections
        .iter()
        .filter(|(p1, p2)| simple_overlap(p1, p2))
        .count();
    println!("Numbr of simple overlaps: {}", res);
}
