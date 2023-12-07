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
pub fn part2(){
    let lines: Vec<_> = include_str!("./day4.txt")
    .lines().collect();
    let count = lines.len();
    let mut ticket_vec =  vec![1; count];
    let mut index = 0;

    while index < count{
        let mut num_matches = 0;
        let [winners, havers] = lines[index].split(": ")
        .nth(1)
        .unwrap()
        .split(" | ")
        .collect::<Vec<_>>()[..] else { todo!() };
        let win_vec: Vec<_> = winners.trim().split_whitespace().collect();
        let have_vec: Vec<_> = havers.trim().split_whitespace().collect();
        for val in have_vec{
            if win_vec.contains(&val){
                num_matches += 1;
            }
        }
        let mut iterator = 0;
        while iterator < num_matches{
            ticket_vec[iterator + index + 1] += ticket_vec[index];
            iterator += 1;
        }
        index += 1;
    }
    let answer: i32 = ticket_vec.iter().sum();
    dbg!(answer);
 }