/*
* Advent of Code 2021
* Day 2a
* @olafsm  
*/

fn main() {
    let input:Vec<&str> = include_str!("../input.txt")
        .lines()
        .collect();

    let mut hor = 0;
    let mut depth = 0;

    for line in input {
        let num = &line[line.len()-1..].trim().parse::<i32>().expect("Please type a number!");
        match line[..line.len() - 2].as_ref() {
            "forward"   => hor += num,
            "up"        => depth -= num,
            "down"      => depth += num,
            _ =>{}
        }
    }
    println!("Hor: {}, \nDepth: {}", hor, depth);
    println!("Product: {}", hor * depth);
}