
pub fn part1() {
    let mut total = 0;
    let input_str = include_str!("./day1.txt");
    let lines = input_str.split("\n");
    for line in lines {
        let mut int_array = line.chars().filter(|c| c.is_numeric());
        let first;
        let last ;// dbg!(int_array.collect::<Vec<_>>());
        if int_array.clone().count() == 1{
            first = int_array.next().unwrap().to_digit(10).unwrap();
            last = first;
        }
        else{
            first = int_array.next().unwrap().to_digit(10).unwrap();
            last = int_array.last().unwrap().to_digit(10).unwrap();
        }

        total += first * 10 + last;
    }
    dbg!(total);
}
