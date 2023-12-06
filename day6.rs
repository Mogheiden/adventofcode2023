pub fn part1()->i32{
    let distances = [334, 1135, 1350, 2430];
    let times = [56, 71, 79, 99];
    let mut part1 = 1;
    
    for i in 0..times.len(){
        let cur_time = times[i];
        let cur_dist = distances[i];
        let mut min = 0;
        let mut max = 0;

        for j in 0..cur_time{
            let distance = (cur_time - j) * j;
            if distance > cur_dist{
                min = j;
                break;
            }
        }
        let mut j = cur_time;
        while j > 0 {
            let distance = (cur_time - j) * j;
            if distance > cur_dist{
                max = j;
                break;
            }
            j -= 1;
        }
        let local = max - min + 1;
        part1 *= local;
    }
    dbg!(part1)
}

pub fn part2(){
    let distance:i64 = 334113513502430;
    let time = 56717999;
    let mut middle = time/2;
    let mut max = 0;
    
    while middle < time {
        let mut cur:i64 = (time - middle) * middle;
        if cur < distance{
            for j in 0..15000{
                let off = middle - j;
                cur = (time - off) * off;
                if cur > distance{
                    max = off;
                    break;
                }
            }
        }
        middle += 15000;
    }
    let min = time - max;
    let part2 = max - min + 1;
    dbg!(part2);
}