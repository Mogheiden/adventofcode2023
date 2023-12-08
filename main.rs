mod day8;

fn main() {
    let start = std::time::Instant::now();
    day8::part1();
    day8::part2();
    println!("Finished in {:?}", start.elapsed());
}
