mod one;

fn main() {
    println!("solution to one pt 1: {}", one::solve1("src/one.txt").unwrap());
    println!("solution to one pt 2: {}", one::solve2("src/one.txt").unwrap());
}
