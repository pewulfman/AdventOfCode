use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn calculate_folders_size(
    folder: &String,
    folders_content: &HashMap<String, Vec<(String, usize)>>,
    size_map: &mut HashMap<String, usize>,
) -> usize {
    let content = folders_content.get(folder).unwrap();
    if size_map.contains_key(folder) {
        return *size_map.get(folder).unwrap();
    };
    let mut total_size = 0;
    for (name, size) in content {
        if *size == 0 {
            if size_map.contains_key(name) {
                total_size += size_map.get(name).unwrap();
            } else {
                calculate_folders_size(name, folders_content, size_map);
                total_size += size_map.get(name).unwrap();
            }
        } else {
            total_size += size;
        }
    }
    size_map.insert(folder.to_string(), total_size);
    return total_size;
}
fn main() {
    let mut stack: Vec<String> = Vec::new();
    let mut folders_content: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    // Analyse folders content and construct the datas
    stdin().lock().lines().for_each(|line| {
        let line = line.unwrap();
        if line.starts_with("$") {
            // Instruction
            if line == "$ cd .." {
                stack.pop();
            } else if line.starts_with("$ cd") {
                let folder = line.split(" ").nth(2).unwrap().to_string();
                stack.push(folder);
            }
        } else {
            let pwd = stack.join("/");
            //folder content
            if line.starts_with("dir") {
                let folder = line.split(" ").nth(1).unwrap().to_string();
                let size = 0;
                folders_content
                    .entry(pwd.to_string())
                    .or_insert_with(Vec::new)
                    .push((pwd + "/" + &folder, size));
            } else {
                let size = line.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
                let file = line.split(" ").nth(1).unwrap().to_string();
                folders_content
                    .entry(pwd.to_string())
                    .or_insert_with(Vec::new)
                    .push((pwd + "/" + &file, size));
            }
        }
    });
    let mut size_map: HashMap<String, usize> = HashMap::new();
    let root = "/".to_string();
    calculate_folders_size(&root, &folders_content, &mut size_map);
    let res: usize = size_map
        .iter()
        .filter(|(_, size)| **size <= 100000)
        .map(|(_, s)| *s)
        .sum();

    println!("sum of sizes of directory of at most 100000: {}", res);

    //part 2
    const TOTAL_SPACE: usize = 70000000;
    const REQUIRED_FREE_SPACE: usize = 30000000;
    const MAX_USED_SPACE: usize = TOTAL_SPACE - REQUIRED_FREE_SPACE;

    let total_used_space: usize = *size_map.get("/").unwrap();
    let space_to_free = total_used_space - MAX_USED_SPACE;
    if space_to_free == 0 {
        println!("no space to free");
        return;
    }
    let res = size_map
        .iter()
        .filter(|(_, size)| **size > space_to_free)
        .map(|(_, s)| *s)
        .min()
        .unwrap();

    println!("space to free: {}", res);
}
