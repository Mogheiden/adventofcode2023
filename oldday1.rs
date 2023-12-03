pub fn part1() {
    let input_str = include_str!("./2021day1.txt");
    let lines: Vec<_> = input_str.split("\n").collect();
    let mut index = 0;
    let mut total = 0;

    while index < lines.len() - 1
    {
        let first = lines[index].parse::<i32>().unwrap();
        let second = lines[index + 1].parse::<i32>().unwrap();

        if first < second{
            total += 1;
        }
        index += 1;
    } 
    dbg!(total);
}

pub fn part2() {
    let input_str = include_str!("./2021day1.txt");
    let lines: Vec<_> = input_str.split("\n").collect();
    let mut index = 0;
    let mut total = 0;

    while index < lines.len() - 3
    {
        let first = lines[index].parse::<i32>().unwrap();
        let second = lines[index + 1].parse::<i32>().unwrap();
        let third = lines[index + 2].parse::<i32>().unwrap();
        let fourth = lines[index + 3].parse::<i32>().unwrap();

        let lower = first + second + third;
        let higher = second + third + fourth;
        if lower < higher{
            total += 1;
        }
        index += 1;
    } 
    dbg!(total);
}