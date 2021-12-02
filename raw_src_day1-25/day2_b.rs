/*
* Advent of Code 2021
* Day 2b
* @olafsm  
*/

fn main() {
    let input:Vec<&str> = include_str!("../input.txt")
        .lines()
        .collect();
    
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input {
        let num = &line[line.len()-1..].trim().parse::<i32>().expect("Error");
        match line[..line.len() - 2].as_ref() {
            "forward"   => {hor += num; depth += aim*num},
            "up"        => aim -= num,
            "down"      => aim += num,
            _ =>{}
        }
    }
    println!("Hor: {}, \nDepth: {}", hor, depth);
    println!("Product: {}", hor * depth);
}