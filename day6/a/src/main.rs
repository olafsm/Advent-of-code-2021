/*
* Advent of Code 2021
* Day 6a
* @olafsm  
*/

fn main() {
    let mut input:Vec<usize> = include_str!("../input.txt")
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut numbers = [0;9];
    for num in input {
        numbers[num] += 1;
    }

    for i in 1..81 {
        let first = numbers[0];
        numbers.rotate_left(1);
        numbers[6] += first;
    }
    let total:usize = numbers.iter().sum();
    println!("{}", total);
}
