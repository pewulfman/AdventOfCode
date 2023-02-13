use std::{
    cell::RefCell,
    io::{stdin, BufRead},
    rc::Rc,
};

use num::integer::lcm;
use regex::Regex;

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Rc<dyn Fn(u64) -> u64>,
    test: u64,
    if_true: usize,
    if_false: usize,
    total: usize,
}

impl Monkey {
    fn new(
        items: Vec<u64>,
        operation: Rc<dyn Fn(u64) -> u64>,
        test: u64,
        if_true: usize,
        if_false: usize,
    ) -> Monkey {
        Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            total: 0,
        }
    }
    pub fn process_turn(&mut self, graph: &[RefCell<Monkey>]) {
        //println!("{} items", self.items.len());
        self.total += self.items.len();
        for item in self.items.drain(..) {
            let op = &self.operation;
            let item = op(item) / 3;
            let target = if item % self.test == 0 {
                self.if_true
            } else {
                self.if_false
            };
            graph[target].borrow_mut().items.push(item);
        }
    }
    pub fn process_turn_2(&mut self, ppcm: u64, graph: &[RefCell<Monkey>]) {
        //println!("{} items", self.items.len());
        self.total += self.items.len();
        for item in self.items.drain(..) {
            let op = &self.operation;
            let item = op(item) % ppcm;
            let target = if item % self.test == 0 {
                self.if_true
            } else {
                self.if_false
            };
            graph[target].borrow_mut().items.push(item);
        }
    }
}

fn parse_input() -> Vec<RefCell<Monkey>> {
    stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .chunks(7)
        .enumerate()
        .map(|(monkey_idx, chunk)| {
            let r = Regex::new(r"(\d+)").unwrap();
            let monkey = r.captures(&chunk[0]).unwrap()[1].parse::<usize>().unwrap();
            assert_eq!(monkey, monkey_idx);
            let items = r
                .find_iter(&chunk[1])
                .map(|m| m.as_str().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let test = r.captures(&chunk[3]).unwrap()[1].parse::<u64>().unwrap();
            let if_true = r.captures(&chunk[4]).unwrap()[1].parse::<usize>().unwrap();
            let if_false = r.captures(&chunk[5]).unwrap()[1].parse::<usize>().unwrap();
            let r = Regex::new(r"(new) = (old|\d+) ([+*-/]) (old|\d+)").unwrap();
            let captures = r.captures(&chunk[2]).unwrap();
            let capture_op = |capture: &str| -> Box<dyn Fn(u64) -> u64> {
                if capture == "old" {
                    Box::new(|x| x)
                } else {
                    let ex: u64 = capture.parse().unwrap();
                    Box::new(move |_| ex)
                }
            };
            let fn1 = capture_op(&captures[2]);
            let fn2 = capture_op(&captures[4]);
            let operation: Rc<dyn Fn(u64) -> u64> = match captures[3].as_ref() {
                "+" => Rc::new(move |x| fn1(x) + fn2(x)),
                "-" => Rc::new(move |x| fn1(x) - fn2(x)),
                "*" => Rc::new(move |x| fn1(x) * fn2(x)),
                "/" => Rc::new(move |x| fn1(x) / fn2(x)),
                _ => panic!("Unknown operation"),
            };
            RefCell::new(Monkey::new(items, operation, test, if_true, if_false))
        })
        .collect()
}
fn main() {
    println!("Day 11!");
    let mut monkeys = parse_input();
    println!("parsed!");
    let mut monkeys2 = monkeys.clone();
    for _ in 0..20 {
        monkeys.iter().for_each(|monkey| {
            monkey.borrow_mut().process_turn(&monkeys);
        })
    }
    println!(
        "totals {:?}",
        monkeys.iter().map(|m| m.borrow().total).collect::<Vec<_>>()
    );

    monkeys.sort_by_key(|m| std::cmp::Reverse(m.borrow().total));
    let res: usize = monkeys[0..2]
        .iter()
        .map(|monkey| monkey.borrow().total)
        .product();
    println!("Part 1 {}", res);
    std::mem::drop(monkeys);

    println!("Part 2");
    let ppcm = monkeys2
        .iter()
        .map(|m| m.borrow().test)
        .reduce(lcm)
        .unwrap();

    for _ in 0..10000 {
        monkeys2.iter().for_each(|monkey| {
            monkey.borrow_mut().process_turn_2(ppcm, &monkeys2);
        })
    }
    println!(
        "totals {:?}",
        monkeys2
            .iter()
            .map(|m| m.borrow().total)
            .collect::<Vec<_>>()
    );

    monkeys2.sort_by_key(|m| std::cmp::Reverse(m.borrow().total));
    let res: usize = monkeys2[0..2]
        .iter()
        .map(|monkey| monkey.borrow().total)
        .product();
    println!("Part 1 {}", res);
}
