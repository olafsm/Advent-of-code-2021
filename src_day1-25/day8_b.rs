/*
* Advent of Code 2021
* Day 8a
* @olafsm  
*/

use std::collections::HashMap;
use itertools::Itertools;
fn main() {
    let input:Vec<Vec<Vec<String>>> = include_str!("../input.txt")
        .lines()
        .map(|x| {
            x.split(" | ")
                .map(|y| {
                    y.split(" ").map(
                        |z| z.chars().sorted().collect::<String>()
                    ).collect()
                })
                .collect()
        })
        .collect();
        
    let mut total = 0;
    for line in input {
        total += get_output_value(line[0].clone(), line[1].clone());
    }
    println!("Total: {}", total);
}

fn get_output_value(mut input:Vec<String>, output:Vec<String>)  -> i32 {

    input.sort_by(|a, b| a.len().cmp(&b.len()));

    let mut characters = HashMap::new();
    
    // 1, 4, 7 and 8 are unique by length
    characters.insert(input[0].to_string(), "1");
    characters.insert(input[1].to_string(), "7");
    characters.insert(input[2].to_string(), "4");
    characters.insert(input[9].to_string(), "8");
    
    // Find 0, 6 and 9
    for x in &input[6..9] {

        let mut count = 0;
        for letter in x.chars() {
            if !input[2].contains(letter) {
                count +=1;
            }
        }
        match count {
            2 => { characters.insert(x.to_string(), "9"); },
            3 => { characters.insert(x.to_string(), "0"); },
            _ => {},
        };

        if input[0].chars().any(|letter| !x.contains(letter)) {
            characters.insert(x.to_string(), "6");
        }
    }

    // find 2, 3 & 5
    for x in &input[3..6] {
        let mut count = 0;
        for letter in x.chars() {
            if !input[2].contains(letter){
                count +=1;
            }
        }
        match count {
            2 => { characters.insert(x.to_string(), "5"); },
            3 => { characters.insert(x.to_string(), "2"); },
            _ => {},
        };

        if input[0].chars().all(|letter| x.contains(letter)) {
            characters.insert(x.to_string(), "3");
        } 
    }

    // Get values from HashMap and return integer
    let mut completed_string = "".to_string();
    for x in output {
        completed_string += characters.get(&x).unwrap();
    }
    completed_string.parse::<i32>().unwrap()
}