pub fn part1() {
    let output = include_str!("./day4.txt")
    .lines()
    .fold(0, |points, line| {
        let [winners, havers] = line.split(": ")
        .nth(1)
        .unwrap()
        .split(" | ")
        .collect::<Vec<_>>()[..] else { todo!() };
        let win_vec: Vec<_> = winners.trim().split_whitespace().collect();
        let have_vec: Vec<_> = havers.trim().split_whitespace().collect();
        let mut line_points = 0;

        for val in have_vec{
            if win_vec.contains(&val) {
                if line_points == 0{
                    line_points = 1;
                }
                else{
                    line_points *= 2;
                }
            }
        }
        return points + line_points;
    });
    dbg!(output);
}