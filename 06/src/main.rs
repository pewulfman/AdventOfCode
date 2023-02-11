use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn has_duplicates(window: &[u8]) -> bool {
    let mut seen: HashSet<u8> = HashSet::new();
    for i in window {
        if seen.contains(i) {
            return true;
        }
        seen.insert(*i);
    }
    return false;
}
fn main() {
    let mut buf: Vec<u8> = Vec::new();
    let _size = stdin().read_to_end(&mut buf).unwrap();
    let mut res = 0;
    for (i, window) in buf.windows(4).enumerate() {
        if !has_duplicates(window) {
            res = i + 4;
            break;
        }
    }
    println!("bytes before packet: {}", res);
    for (i, window) in buf.windows(14).enumerate() {
        if !has_duplicates(window) {
            res = i + 14;
            break;
        }
    }
    println!("bytes before message: {}", res);
}
