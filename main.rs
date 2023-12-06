mod day6;

fn main(){
    let start = std::time::Instant::now();
    day6::part1();
    day6::part2();
    println!("Finished in {:?}", start.elapsed());
}