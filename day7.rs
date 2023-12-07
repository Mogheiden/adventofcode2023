enum Handtype {
    HighCard,
    TwoOfKind,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

pub fn part1() {
    const STRENGTH_ORDER: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    fn get_type(hand: &str) -> i32 {
        let mut array: [i32; 13] = [0; 13];
        for char in hand.chars() {
            let index = STRENGTH_ORDER
                .iter()
                .position(|&card| card == char)
                .unwrap();
            array[index] += 1;
        }
        let maximum = *array.iter().max().unwrap();
        if maximum == 5 {
            return Handtype::FiveOfKind as i32;
        }
        if maximum == 4 {
            return Handtype::FourOfKind as i32;
        }
        if maximum == 3 {
            if array.contains(&2) {
                return Handtype::FullHouse as i32;
            }
            return Handtype::ThreeOfKind as i32;
        }
        if maximum == 2 {
            let num2s = array.iter().fold(0, |cum, &val| {
                if val == 2 {
                    return cum + 1;
                } else {
                    return cum;
                }
            });
            if num2s == 2 {
                return Handtype::TwoPair as i32;
            } else {
                return Handtype::TwoOfKind as i32;
            }
        }
        if maximum == 1 {
            return Handtype::HighCard as i32;
        } else {
            return -1;
        }
    }

    let mut hand_buckets = std::iter::repeat(vec![])
        .take(7)
        .collect::<Vec<Vec<(&str, i32)>>>();
    let lines: Vec<_> = include_str!("./day7.txt")
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            return (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            );
        })
        .collect();

    for hand in lines {
        let hb_index = get_type(hand.0);
        hand_buckets[hb_index as usize].push(hand);
    }

    for mut vector in hand_buckets.iter_mut() {
        vector.sort_unstable_by(|a, b| {
            let card1 = a.0;
            let card2 = b.0;
            for i in 0..card1.len() {
                let card1char = card1.chars().nth(i).unwrap();
                let card2char = card2.chars().nth(i).unwrap();
                let pos1 = STRENGTH_ORDER
                    .iter()
                    .position(|&card| card == card1char)
                    .unwrap();
                let pos2 = STRENGTH_ORDER
                    .iter()
                    .position(|&card| card == card2char)
                    .unwrap();

                if pos2 == pos1 {
                    continue;
                }

                return pos2.cmp(&pos1);
            }
            panic!();
        })
    }
    let mut part1score = 0;
    let mut part1mult = 1;
    for vector in hand_buckets {
        for hand in vector {
            part1score += hand.1 * part1mult;
            part1mult += 1;
        }
    }
    dbg!(part1score);
}

pub fn part2() {
    const STRENGTH_ORDER: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];
    fn get_type(hand: &str) -> i32 {
        let mut jkr = 0;
        let mut array: [i32; 12] = [0; 12];
        for char in hand.chars() {
            let index = STRENGTH_ORDER
                .iter()
                .position(|&card| card == char)
                .unwrap();
            if index == 12 {
                jkr += 1;
            } else {
                array[index] += 1;
            }
        }
        let maximum = *array.iter().max().unwrap() + jkr;
        if maximum == 5 {
            return Handtype::FiveOfKind as i32;
        }
        if maximum == 4 {
            return Handtype::FourOfKind as i32;
        }
        if maximum == 3 {
            let num2s = array.iter().fold(0, |cum, &val| {
                if val == 2 {
                    return cum + 1;
                } else {
                    return cum;
                }
            });
            let num3s = array.iter().fold(0, |cum, &val| {
                if val == 3 {
                    return cum + 1;
                } else {
                    return cum;
                }
            });
            if num2s == 2 || (num2s == 1 && num3s == 1) {
                return Handtype::FullHouse as i32;
            }
            return Handtype::ThreeOfKind as i32;
        }
        if maximum == 2 {
            let num2s = array.iter().fold(0, |cum, &val| {
                if val == 2 {
                    return cum + 1;
                } else {
                    return cum;
                }
            });
            if num2s == 2 {
                return Handtype::TwoPair as i32;
            } else {
                return Handtype::TwoOfKind as i32;
            }
        }
        if maximum == 1 {
            return Handtype::HighCard as i32;
        } else {
            return -1;
        }
    }

    let mut hand_buckets = std::iter::repeat(vec![])
        .take(7)
        .collect::<Vec<Vec<(&str, i32)>>>();
    let lines: Vec<_> = include_str!("./day7.txt")
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            return (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            );
        })
        .collect();

    for hand in lines {
        let hb_index = get_type(hand.0);
        hand_buckets[hb_index as usize].push(hand);
    }

    for mut vector in hand_buckets.iter_mut() {
        vector.sort_unstable_by(|a, b| {
            let card1 = a.0;
            let card2 = b.0;
            for i in 0..card1.len() {
                let card1char = card1.chars().nth(i).unwrap();
                let card2char = card2.chars().nth(i).unwrap();
                let pos1 = STRENGTH_ORDER
                    .iter()
                    .position(|&card| card == card1char)
                    .unwrap();
                let pos2 = STRENGTH_ORDER
                    .iter()
                    .position(|&card| card == card2char)
                    .unwrap();

                if pos2 == pos1 {
                    continue;
                }

                return pos2.cmp(&pos1);
            }
            panic!();
        })
    }
    let mut part1score = 0;
    let mut part1mult = 1;
    for vector in hand_buckets {
        for hand in vector {
            part1score += hand.1 * part1mult;
            part1mult += 1;
        }
    }
    dbg!(part1score);
}
