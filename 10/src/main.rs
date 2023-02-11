use std::io::{stdin, BufRead};

fn display(screen: &mut Vec<char>, cycle: usize, x: i32) {
    let horizontal: i32 = (cycle as i32) % 40;
    if (horizontal - x).abs() <= 1 {
        screen.push('#');
    } else {
        screen.push('.');
    }
}
fn main() {
    println!("Day 10!");
    let instruct = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    println!("Part 1");
    let mut x = 1;
    let mut cycle = 0;
    let mut threshold = 20;
    let mut power = Vec::with_capacity(6);
    instruct.iter().for_each(|line| {
        let old_x = x;
        if line.starts_with("noop") {
            cycle += 1;
        } else if line.starts_with("addx") {
            cycle += 2;
            x = x + line.split(" ").last().unwrap().parse::<i32>().unwrap();
        } else {
            panic!("Unknown instruction");
        }
        if cycle >= threshold {
            power.push(old_x * threshold);
            threshold += 40;
        }
    });

    assert_eq!(power.len(), 6);
    assert_eq!(threshold, 260);
    println!("{}", power.iter().sum::<i32>());

    println!("Part 2");
    let mut x = 1;
    let mut cycle = 0;
    let mut screen = Vec::with_capacity(240);
    instruct.iter().for_each(|line| {
        if line.starts_with("noop") {
            display(&mut screen, cycle, x);
            cycle += 1;
        } else if line.starts_with("addx") {
            display(&mut screen, cycle, x);
            cycle += 1;
            display(&mut screen, cycle, x);
            cycle += 1;
            x = x + line.split(" ").last().unwrap().parse::<i32>().unwrap();
        } else {
            panic!("Unknown instruction");
        }
    });
    println!("screen :");
    screen
        .chunks(40)
        .for_each(|f| println!("{}", f.iter().collect::<String>()));
}
