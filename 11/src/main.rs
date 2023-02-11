use std::io::{stdin, BufRead};

use regex::Regex;

struct Monkey {
    items: Vec<u8>,
    operation: Box<dyn Fn(u8) -> u8>,
    test: u8,
    total: u32,
}

impl Monkey {
    fn new(items: Vec<u8>, operation: &'static dyn Fn(u8) -> u8, test: u8) -> Monkey {
        Monkey {
            items,
            operation: Box::new(operation),
            test,
            total: 0,
        }
    }
    pub fn push(&mut self, items: &Vec<u8>) {
        self.items.extend(items);
    }
    pub fn process_turn(&mut self, output: &mut Vec<Vec<u8>>, right: usize, left: usize) {
        let mut items = Vec::new();
        std::mem::swap(&mut self.items, &mut items);
        for item in items {
            let op = &self.operation;
            let item = op(item) / 3;
            if item % self.test == 0 {
                output[right].push(item);
            } else {
                output[left].push(item);
            }
        }
        self.total += self.items.len() as u32;
    }
}

fn parse_input() -> (Vec<Monkey>, Vec<(usize, usize, usize)>) {
    stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .chunks(7)
        .map(|chunk| {
            let r = Regex::new(r"(\d+)").unwrap();
            let monkey = r.captures(&chunk[0]).unwrap()[1].parse::<usize>().unwrap();
            let items = chunk[1]
                .split_whitespace()
                .filter_map(|x| x.parse::<u8>().ok())
                .collect::<Vec<u8>>();
            let test = r.captures(&chunk[3]).unwrap()[1].parse::<u8>().unwrap();
            let right = r.captures(&chunk[4]).unwrap()[1].parse::<usize>().unwrap();
            let left = r.captures(&chunk[5]).unwrap()[1].parse::<usize>().unwrap();
            let r = Regex::new(r"(new) = (old|\d+) ([+*-/]) (old|\d+)").unwrap();
            let captures = r.captures(&chunk[2]).unwrap();
            let capture_op: fn(&String, &mut u8) -> Box<dyn Fn(u8) -> u8> = |capture, c| {
                if capture == "old" {
                    Box::new(|x| x)
                } else {
                    let ex: u8 = capture.parse().unwrap();
                    *c = ex;
                    Box::new(|_| *c)
                }
            };
            let mut c1 = 0;
            let fn1 = capture_op(&captures[2].into(), &mut c1);
            let mut c2 = 0;
            let fn2 = capture_op(&captures[4].into(), &mut c2);
            let operation: Box<dyn Fn(u8) -> u8> = match captures[3].as_ref() {
                "+" => Box::new(|x| fn1(x) + fn2(x)),
                "-" => Box::new(|x| fn1(x) - fn2(x)),
                "*" => Box::new(|x| fn1(x) * fn2(x)),
                "/" => Box::new(|x| fn1(x) / fn2(x)),
                _ => panic!("Unknown operation"),
            };
            (Monkey::new(items, &operation, test), (monkey, right, left))
        })
        .unzip()
}
fn main() {
    println!("Day 11!");
    let (mut monkeys, edges) = parse_input();
    let mut output: Vec<Vec<u8>> = monkeys.iter().map(|_| Vec::new()).collect();
    for _ in 0..20 {
        edges.iter().for_each(|(m, r, l)| {
            let monkey = &mut monkeys[*m];
            monkey.push(&output[*m]);
            monkey.process_turn(&mut output, *r, *l)
        })
    }

    monkeys.sort_by(|a, b| a.total.cmp(&b.total));
    let res: u32 = monkeys[0..1].iter().map(|monkey| monkey.total).product();
    println!("Part 1 {}", res)
}
