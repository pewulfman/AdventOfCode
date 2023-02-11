use std::io::{stdin, BufRead};

fn main() {
    let forest: Vec<Vec<usize>> = stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // values are max_size, and index
    let mut max_north: Vec<usize> = vec![0; forest[0].len()];
    let mut max_west: Vec<usize> = vec![0; forest.len()];

    let forest_half_parse: Vec<Vec<(usize, bool)>> = forest
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, c)| {
                    let visible_north = if i == 0 {
                        max_north[j] = *c;
                        true
                    } else {
                        let max = max_north[j];
                        if *c > max {
                            max_north[j] = *c;
                            true
                        } else {
                            false
                        }
                    };
                    let visible_west = if j == 0 {
                        max_west[i] = *c;
                        true
                    } else {
                        let max = max_west[i];
                        if *c > max {
                            max_west[i] = *c;
                            true
                        } else {
                            false
                        }
                    };
                    (*c, visible_north || visible_west)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut max_south: Vec<usize> = vec![0; forest[0].len()];
    let mut max_east: Vec<usize> = vec![0; forest.len()];
    let forest_full_parse = forest_half_parse
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .rev()
                .enumerate()
                .map(|(j, (c, visible))| {
                    let visible_south = if i == 0 {
                        max_south[j] = *c;
                        true
                    } else {
                        let max = max_south[j];
                        if *c > max {
                            max_south[j] = *c;
                            true
                        } else {
                            false
                        }
                    };
                    let visible_east = if j == 0 {
                        max_east[i] = *c;
                        true
                    } else {
                        let max = max_east[i];
                        if *c > max {
                            max_east[i] = *c;
                            true
                        } else {
                            false
                        }
                    };
                    (c, *visible || visible_south || visible_east)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let total_visible_trees = forest_full_parse
        .iter()
        .map(|row| row.iter().filter(|(_, visible)| *visible).count())
        .sum::<usize>();
    println!("Number of visible trees {}", total_visible_trees);

    //part2
    // linked list of size and position, keep track of what we see as we go along
    let mut visible_at_north: Vec<Vec<(usize, usize)>> =
        vec![Vec::with_capacity(forest[0].len()); forest.len()];
    let mut visible_at_west: Vec<Vec<(usize, usize)>> =
        vec![Vec::with_capacity(forest.len()); forest[0].len()];

    let parse_half_forest = forest
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(|(j, c)| {
                    let north = {
                        visible_at_north[j].retain(|(size, _index)| *size >= c);
                        let top = visible_at_north[j].pop();
                        match top {
                            Some((size, index)) => {
                                if c == size {
                                    visible_at_north[j].push((c, i));
                                } else {
                                    visible_at_north[j].push((size, index));
                                    visible_at_north[j].push((c, i));
                                }
                                i - index
                            }
                            None => {
                                visible_at_north[j].push((c, i));
                                i
                            }
                        }
                    };
                    let west = {
                        visible_at_west[i].retain(|(size, _index)| *size >= c);
                        let top = visible_at_west[i].pop();
                        match top {
                            Some((size, index)) => {
                                if c == size {
                                    visible_at_west[i].push((c, j));
                                } else {
                                    visible_at_west[i].push((size, index));
                                    visible_at_west[i].push((c, j));
                                }
                                j - index
                            }
                            None => {
                                visible_at_west[i].push((c, j));
                                j
                            }
                        }
                    };
                    (c, [north, west])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut visible_at_south: Vec<Vec<(usize, usize)>> =
        vec![Vec::with_capacity(forest_half_parse[0].len()); forest_half_parse.len()];
    let mut visible_at_east: Vec<Vec<(usize, usize)>> =
        vec![Vec::with_capacity(forest_half_parse.len()); forest_half_parse[0].len()];

    let parse_full_forest = parse_half_forest
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, row)| {
            row.into_iter()
                .rev()
                .enumerate()
                .map(|(j, (c, views))| {
                    let south = {
                        visible_at_south[j].retain_mut(|(size, _index)| *size >= c);
                        let top = visible_at_south[j].pop();
                        match top {
                            Some((size, index)) => {
                                if c == size {
                                    visible_at_south[j].push((c, i));
                                } else {
                                    visible_at_south[j].push((size, index));
                                    visible_at_south[j].push((c, i));
                                }
                                i - index
                            }
                            None => {
                                visible_at_south[j].push((c, i));
                                i
                            }
                        }
                    };
                    let east = {
                        visible_at_east[i].retain(|(size, _index)| *size >= c);
                        let top = visible_at_east[i].pop();
                        match top {
                            Some((size, index)) => {
                                if c == size {
                                    visible_at_east[i].push((c, j));
                                } else {
                                    visible_at_east[i].push((size, index));
                                    visible_at_east[i].push((c, j));
                                }
                                j - index
                            }
                            None => {
                                visible_at_east[i].push((c, j));
                                j
                            }
                        }
                    };
                    (c, [views[0], views[1], south, east])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let total_visible_trees = parse_full_forest
        .iter()
        .map(|row| {
            row.iter()
                .map(|(_, views)| views.iter().product::<usize>())
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Max visible trees {}", total_visible_trees);
}
