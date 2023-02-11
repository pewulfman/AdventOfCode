use ndarray::{arr1, ArrayBase, Ix1, OwnedRepr};
use regex::Regex;
use std::{
    cmp::max,
    io::{stdin, BufRead},
};

fn calculate_next_knot_pos(
    pos: &Vec<ArrayBase<OwnedRepr<i32>, Ix1>>,
) -> Vec<ArrayBase<OwnedRepr<i32>, Ix1>> {
    let mut current_pos = arr1(&[0, 0]);
    // values are max_size, and index
    pos.iter()
        .map(|h_pos| {
            let vector = h_pos - &current_pos;
            let distance = vector.to_vec().iter().map(|x: &i32| x.abs()).max().unwrap();
            if distance > 1 {
                let norm_vector = arr1(&[
                    vector[0] / max(vector[0].abs(), 1),
                    vector[1] / max(vector[1].abs(), 1),
                ]);
                current_pos = &current_pos + norm_vector;
                arr1(&current_pos.to_vec())
            } else {
                arr1(&current_pos.to_vec())
            }
        })
        .collect()
}

fn main() {
    println!("Solving day 9!");
    let movements: Vec<(char, u32)> = stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let r = Regex::new(r"([UDLR]) ([0-9]+)").unwrap();
            let caps = r.captures(&line).unwrap();
            (
                caps.get(1).unwrap().as_str().chars().next().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect::<Vec<(char, u32)>>();

    // values are max_size, and index
    let mut h_pos = vec![arr1(&[0, 0])];
    movements.iter().for_each(|(direction, step)| {
        let vector = match direction {
            'U' => arr1(&[0, 1]),
            'D' => arr1(&[0, -1]),
            'L' => arr1(&[-1, 0]),
            'R' => arr1(&[1, 0]),
            _ => panic!(),
        };
        for _ in 0..*step {
            let old_h = h_pos.last().unwrap();
            let new_h = old_h + &vector;
            h_pos.push(new_h);
        }
    });
    let mut t_pos = calculate_next_knot_pos(&h_pos);
    t_pos.sort_by(|a, b| {
        if a[0] == b[0] {
            a[1].cmp(&b[1])
        } else {
            a[0].cmp(&b[0])
        }
    });
    t_pos.dedup();
    println!("Part 1: {}", t_pos.len());

    println!("Part 2");
    let k1_pos = calculate_next_knot_pos(&h_pos);
    let k2_pos = calculate_next_knot_pos(&k1_pos);
    let k3_pos = calculate_next_knot_pos(&k2_pos);
    let k4_pos = calculate_next_knot_pos(&k3_pos);
    let k5_pos = calculate_next_knot_pos(&k4_pos);
    let k6_pos = calculate_next_knot_pos(&k5_pos);
    let k7_pos = calculate_next_knot_pos(&k6_pos);
    let k8_pos = calculate_next_knot_pos(&k7_pos);
    let k9_pos = calculate_next_knot_pos(&k8_pos);

    let mut t_pos = k9_pos.to_vec();
    t_pos.sort_by(|a, b| {
        if a[0] == b[0] {
            a[1].cmp(&b[1])
        } else {
            a[0].cmp(&b[0])
        }
    });
    t_pos.dedup();
    println!("Res : {}", t_pos.len());
}
