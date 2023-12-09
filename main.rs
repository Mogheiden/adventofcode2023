mod day9;

fn main() {
    let start = std::time::Instant::now();
    day9::part1();
    day9::part2();
    println!("Finished in {:?}", start.elapsed());
}
