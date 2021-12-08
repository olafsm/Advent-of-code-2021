/*
* Advent of Code 2021
* Day 8a
* @olafsm  
*/

fn main() {
    let input:Vec<Vec<Vec<&str>>> = include_str!("../input.txt")
        .lines()
        .map(|x| {
            x.split(" | ")
                .map(|y| y.split(" ").collect())
                .collect()
        })
        .collect();

    let mut count = 0;
    
    for x in input {
        for y in &x[1] {
            match y.len() {
                2 | 3 |4 | 7 => {count+=1},
                _ => {}
            }
        }
    }
    println!("{}", count);
}