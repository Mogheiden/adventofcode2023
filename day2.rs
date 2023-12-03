struct CubeCounts {
    red: i32, green: i32, blue: i32,
}

const MAX_DICE: CubeCounts = CubeCounts { blue: 14, green: 13, red: 12 };

pub fn part1() {
    let input_str = include_str!("./day2.txt");

    let output = input_str.split("\n").filter_map(|line| {
            let (game_str, sets_str) = {
                let mut line_split = line.split(": ");
                (line_split.next().unwrap(), line_split.next().unwrap())
            };
            
            let game_id = game_str.split(" ").last().unwrap().parse::<i32>().unwrap();
            let is_game_valid = sets_str.split("; ").all(|set_str| {
                let mut cubes = CubeCounts { blue: 0, red: 0, green: 0};
                for cube_count_str in set_str.split(", ") {
                    let (amount, color) = {
                        let mut split = cube_count_str.split(" ");
                        (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap())
                    };
                    match color {
                        "red" => cubes.red = amount,
                        "green" => cubes.green = amount,
                        "blue" => cubes.blue = amount,
                        _ => panic!(),
                    };
                }
                let is_set_valid =
                    cubes.blue <= MAX_DICE.blue &&
                    cubes.green <= MAX_DICE.green &&
                    cubes.red <= MAX_DICE.red;
                return is_set_valid;
            });

            if is_game_valid {Some(game_id)} else {None}
        }).reduce(|acc, curr| acc + curr).unwrap();

    dbg!(output);
}

pub fn part2() {
    let input_str = include_str!("./day2.txt");

    let output = input_str.split("\n").map(|line| {
            let sets_str = {
                let mut line_split = line.split(": ");
                line_split.nth(1).unwrap()
            };
            let mut min_cubes = CubeCounts { blue: 0, red: 0, green: 0};
            sets_str.split("; ").for_each(|set_str| {
                for cube_count_str in set_str.split(", ") {
                    let (amount, color) = {
                        let mut split = cube_count_str.split(" ");
                        (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap())
                    };
                    match color {
                        "red" => if min_cubes.red < amount {
                            min_cubes.red = amount},
                        "green" => if min_cubes.green < amount{
                            min_cubes.green = amount},
                        "blue" => if min_cubes.blue < amount{
                            min_cubes.blue = amount},
                        _ => panic!(),
                    };
                }
            });
            min_cubes.blue * min_cubes.red * min_cubes.green
        }).reduce(|acc, curr| acc + curr).unwrap();

    dbg!(output);
}