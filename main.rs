mod day7;

fn main() {
    let start = std::time::Instant::now();
    day7::part1();
    day7::part2();
    println!("Finished in {:?}", start.elapsed());
}
