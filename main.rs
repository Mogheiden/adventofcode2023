mod day11;

fn main() {
    let start = std::time::Instant::now();
    day11::part1();
    // day10::part2();
    println!("Finished in {:?}", start.elapsed());
}
