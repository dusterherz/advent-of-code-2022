use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut total_score_by_shapes = 0;
    let mut total_score_by_win = 0;
    let score_by_shapes = HashMap::from([
        ('A', HashMap::from([('X', 4), ('Y', 8), ('Z', 3)])),
        ('B', HashMap::from([('X', 1), ('Y', 5), ('Z', 9)])),
        ('C', HashMap::from([('X', 7), ('Y', 2), ('Z', 6)])),
    ]);
    let score_by_win = HashMap::from([
        ('A', HashMap::from([('X', 3), ('Y', 4), ('Z', 8)])),
        ('B', HashMap::from([('X', 1), ('Y', 5), ('Z', 9)])),
        ('C', HashMap::from([('X', 2), ('Y', 6), ('Z', 7)])),
    ]);

    input.split('\n').for_each(|line| {
        if line.len() < 3 {
            return;
        };
        let input_opponent = line.chars().nth(0).unwrap();
        let input_user = line.chars().nth(2).unwrap();
        total_score_by_shapes += score_by_shapes[&input_opponent][&input_user];
        total_score_by_win += score_by_win[&input_opponent][&input_user];
    });

    println!("Total score by shapes {}", total_score_by_shapes);
    println!("Total score by win {}", total_score_by_win);
}
