use std::fs;

fn main() {
    println!("--- Day 1: Find the elves who carries the most calories ---");

    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut all_elves: Vec<i32> = Vec::new();
    let mut current = 0;
    input.split('\n').for_each(|line| {
        if line == "" {
            all_elves.push(current);
            current = 0;
        } else {
            current += match line.parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
        }
    });
    all_elves.sort();
    all_elves.reverse();

    println!(
        "The most calories for each elves is: {:?}",
        all_elves.get(0).expect("No elves found")
    );
    println!(
        "The most calories for 3 first elves is: {:?}",
        all_elves.get(0).expect("No elves found")
            + all_elves.get(1).expect("Elf 2 not found")
            + all_elves.get(2).expect("Elf 3 not found")
    );
}
