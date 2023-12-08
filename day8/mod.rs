use std::collections::{HashMap, HashSet};

const INSTRUCTIONSTR: &[u8] = "LRRLLRLRRRLRRRLRRLRRRLRRLRRRLRRLRRRLRLRRRLRRRLRRRLRLRRLRRRLRRRLRRLRRLRRLRLLLRRRLRRRLRLRLRRLLRRRLRRLRRRLRLRRLRRRLRRRLLRLRLLRRRLRRRLLRRRLRRRLRRRLRRLRRRLLLRRRLRLLLRLRLRLLRLRLLLRRLRRLLRRLRRRLRRLRRLRLRRLLRRLRLRRLLLRRRLLRRRLLRLRLLRRRLRLLRRLRLRRLRLRRRLLRRRLLRRLRLRRLRRLLRLRLRRRLRLRRRR".as_bytes();
// const INSTRUCTIONSTR: &[u8] = "LLR".as_bytes();

struct Node {
    name: String,
    left_id: String,
    right_id: String,
}

pub fn part1() {
    let linesiter = include_str!("./day8.txt").lines();
    let nodes = linesiter.fold(HashMap::new(), |mut map, line| {
        let mut linestr = line.to_string();
        linestr.retain(|c| !"()=,".contains(c));
        let mut field_iter = linestr.split_whitespace();
        let id = field_iter.next().unwrap().to_string();
        let left = field_iter.next().unwrap().to_string();
        let right = field_iter.next().unwrap().to_string();
        map.insert(
            id.clone(),
            Node {
                name: id,
                left_id: left,
                right_id: right,
            },
        );
        map
    });
    let mut numstep = 0;
    let length = INSTRUCTIONSTR.len();
    let mut cur_node = nodes.get("AAA").unwrap();

    while cur_node.name != "ZZZ" {
        let instruction = INSTRUCTIONSTR[numstep % length];
        if instruction == 'L' as u8 {
            cur_node = nodes.get(&cur_node.left_id).unwrap();
        } else {
            cur_node = nodes.get(&cur_node.right_id).unwrap();
        }
        // dbg!(&cur_node.name);
        numstep += 1;
    }
    dbg!(numstep);
}

pub fn part2() {
    let lines_iter = include_str!("./day8.txt").lines();
    let instructions = "LRRLLRLRRRLRRRLRRLRRRLRRLRRRLRRLRRRLRLRRRLRRRLRRRLRLRRLRRRLRRRLRRLRRLRRLRLLLRRRLRRRLRLRLRRLLRRRLRRLRRRLRLRRLRRRLRRRLLRLRLLRRRLRRRLLRRRLRRRLRRRLRRLRRRLLLRRRLRLLLRLRLRLLRLRLLLRRLRRLLRRLRRRLRRLRRLRLRRLLRRLRLRRLLLRRRLLRRRLLRLRLLRRRLRLLRRLRLRRLRLRRRLLRRRLLRRLRLRRLRRLLRLRLRRRLRLRRRR";
    let nodes: HashMap<String, Node> = lines_iter.fold(HashMap::new(), |mut map, line| {
        let mut line_str = line.to_string();
        line_str.retain(|c| !"()=,".contains(c));
        let mut field_iter = line_str.split_whitespace();
        let id = field_iter.next().unwrap().to_string();
        let left_id = field_iter.next().unwrap().to_string();
        let right_id = field_iter.next().unwrap().to_string();
        map.insert(
            id.clone(),
            Node {
                name: id,
                left_id,
                right_id,
            },
        );
        map
    });

    let mut num_steps = 0;
    let mut curr_nodes = nodes
        .values()
        .filter(|node| node.name.ends_with('A'))
        .collect::<Vec<_>>();

    // kudos to andrew for the idea of getting the lcms of loops
    let mut seen_ids = HashMap::<String, HashSet<String>>::new();
    let mut loop_starts = HashMap::<String, i64>::new();
    let mut loop_sizes = HashMap::<String, i64>::new();

    while loop_sizes.len() < curr_nodes.len() {
        for instruction in instructions.chars() {
            num_steps += 1;

            curr_nodes.iter().for_each(|node| {
                if node.name.ends_with('Z')
                    && loop_starts.contains_key(&node.name)
                    && !loop_sizes.contains_key(&node.name)
                {
                    loop_sizes.insert(
                        node.name.clone(),
                        num_steps - loop_starts.get(&node.name).unwrap(),
                    );
                }

                if node.name.ends_with('Z')
                    && !loop_starts.contains_key(&node.name)
                    && !(seen_ids
                        .entry(node.name.clone())
                        .or_insert(HashSet::new())
                        .insert(node.name.clone()))
                {
                    // Found loop - record steps at start
                    loop_starts.insert(node.name.clone(), num_steps);
                }
            });

            curr_nodes = curr_nodes
                .iter()
                .map(|node| match instruction {
                    'L' => nodes.get(&node.left_id).unwrap(),
                    'R' => nodes.get(&node.right_id).unwrap(),
                    _ => panic!(),
                })
                .collect();
        }
    }
    dbg!(loop_sizes.values().fold(1, |acc, curr| lcm(acc, *curr)));
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
