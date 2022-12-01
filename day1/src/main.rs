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
    let first_elf: i32 = all_elves[0];
    let three_first_elves: i32 = all_elves[0..3].iter().sum();

    println!("The most calories for each elves is: {:?}", first_elf);
    println!(
        "The most calories for 3 first elves is: {:?}",
        three_first_elves
    );
}
