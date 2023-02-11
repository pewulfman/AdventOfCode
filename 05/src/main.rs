use std::io::{stdin, BufRead};

fn process_crane_9000(stack: &mut Vec<Vec<char>>, instr: &Vec<String>) {
    for line in instr {
        let mut iter = line.split_whitespace();
        let cmd = iter.next().unwrap();
        let nb = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next(); // skip "from"
        let i = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        iter.next(); // skip "to"
        let j = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        match cmd {
            "move" => {
                for _ in 0..nb {
                    let c = stack[i].pop().unwrap();
                    stack[j].push(c);
                }
            }
            _ => panic!("Unknown command: {}", cmd),
        }
    }
}

fn process_crane_9001(stack: &mut Vec<Vec<char>>, instr: &Vec<String>) {
    for line in instr {
        let mut iter = line.split_whitespace();
        let cmd = iter.next().unwrap();
        let nb = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next(); // skip "from"
        let i = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        iter.next(); // skip "to"
        let j = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        match cmd {
            "move" => {
                let mut s = Vec::with_capacity(nb);
                for _ in 0..nb {
                    let c = stack[i].pop().unwrap();
                    s.push(c);
                }
                for _ in 0..nb {
                    let c = s.pop().unwrap();
                    stack[j].push(c);
                }
            }
            _ => panic!("Unknown command: {}", cmd),
        }
    }
}

fn main() {
    println!("Advent of Code 2022, challenge 5");

    let lines = stdin().lock().lines().map(|l| l.unwrap());
    let (instr, mut init_stack): (Vec<_>, Vec<_>) = lines.partition(|l| l.starts_with("move"));
    init_stack.pop(); // remove the blank line

    let mut stack: Vec<Vec<char>> = init_stack
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|_s| Vec::new())
        .collect();
    init_stack.iter().rev().for_each(|l| {
        let mut iter = l.chars().into_iter();
        let mut i = 0;
        loop {
            iter.next(); // skip "["
            let c = iter.next().unwrap(); // get the char
            if c != ' ' {
                stack[i].push(c); // add it to the stack
            }
            iter.next(); // skip "]"
            if iter.next().is_none() {
                // test for end of line
                break;
            };
            i += 1;
        }
    });
    // Stack is ready, now execute the instructions
    let mut stack2 = stack.clone(); // save a copy for part 2
    process_crane_9000(&mut stack, &instr);
    println!("Final stack craned 9000");
    let top: String = stack.iter().map(|x| x.last().unwrap()).collect();
    println!("{}", top);
    process_crane_9001(&mut stack2, &instr);
    println!("Final stack craned 9001");
    let top: String = stack2.iter().map(|x| x.last().unwrap()).collect();
    println!("{}", top);
}
