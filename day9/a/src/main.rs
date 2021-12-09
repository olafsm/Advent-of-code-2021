/*
* Advent of Code 2021
* Day 9a
* @olafsm  
*/

fn main() {
    let input:Vec<Vec<i32>> = include_str!("../input.txt")
        .lines()
        .map(|x| {
            x.chars().map( |y| {
                y.to_digit(10).unwrap() as i32
            }).collect()
        }).collect();

    let mut total = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, num) in line.iter().enumerate() {
            if i > 0 {
                if num >= &input[i-1][j] {
                    continue;
                }
            }
            if i+1 < input.len()  {
                if num >= &input[i+1][j] {
                    continue;
                }
            }
            if j > 0  {
                if num >= &input[i][j-1] {
                    continue;
                }
            }
            if j+1 < line.len()  {
                if num >= &input[i][j+1] {
                    continue;
                }
            }
            total += num+1;
        }
    }
    eprintln!("total = {:#?}", total);
}