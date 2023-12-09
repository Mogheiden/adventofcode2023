fn diffinder(input_vector: Vec<i32>) -> i32 {
    let mut basecase = true;
    let mut new_vec = Vec::new();
    for i in 1..input_vector.len() {
        let diff = input_vector[i] - input_vector[i - 1];
        if diff != 0 {
            basecase = false;
        }
        new_vec.push(diff);
    }
    // dbg!(&new_vec);
    if basecase == true {
        return *input_vector.last().unwrap();
    } else {
        let ret_num = input_vector.last().unwrap() + diffinder(new_vec);
        // dbg!(ret_num);
        return ret_num;
    }
}

fn backwardsdiffinder(input_vector: Vec<i32>) -> i32 {
    let mut basecase = true;
    let mut new_vec = Vec::new();
    for i in 1..input_vector.len() {
        let diff = input_vector[i] - input_vector[i - 1];
        if diff != 0 {
            basecase = false;
        }
        new_vec.push(diff);
    }
    // dbg!(&new_vec);
    if basecase == true {
        return *input_vector.first().unwrap();
    } else {
        let ret_num = input_vector.first().unwrap() - backwardsdiffinder(new_vec);
        // dbg!(ret_num);
        return ret_num;
    }
}

pub fn part1() {
    let output = include_str!("./day9.txt").lines().fold(0, |cum, line| {
        let num_vec = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let number = diffinder(num_vec);
        // dbg!(number);
        cum + number
    });
    dbg!(output);
}

pub fn part2() {
    let output = include_str!("./day9.txt").lines().fold(0, |cum, line| {
        let num_vec = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let number = backwardsdiffinder(num_vec);
        // dbg!(number);
        cum + number
    });
    dbg!(output);
}
