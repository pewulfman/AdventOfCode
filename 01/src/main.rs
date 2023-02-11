use std::io::BufRead;

fn main() {
    let mut max: i32 = 0;
    let mut sec: i32 = 0;
    let mut thrd: i32 = 0;
    let mut sum: i32 = 0;
    let items: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|line_result| line_result.unwrap())
        .collect();
    items.iter().for_each(|x| match x.parse::<i32>() {
        Ok(x) => sum += x,
        Err(_) => {
            if sum > max {
                thrd = sec;
                sec = max;
                max = sum
            } else if sum > sec {
                thrd = sec;
                sec = sum
            } else if sum > thrd {
                thrd = sum
            }
            sum = 0
        }
    });
    println!("Max {}, Sec {}, third {}", max, sec, thrd);
    println!("total {}", max + sec + thrd);
}
