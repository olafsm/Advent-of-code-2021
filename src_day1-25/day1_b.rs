/*
* Advent of Code 2021
* Day 1b
* @olafsm  
*/

fn main() {
    let input:Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    
    let mut counter = 0;
    let mut prev = -8888;
    let sliding_window = 3;

    for chunk in input.windows(sliding_window) {
        let cur = chunk.iter().sum();
        if (prev != -8888) & (prev < cur) {
            counter += 1;
        }
        prev = cur;
    }
    println!("{}", counter);
}