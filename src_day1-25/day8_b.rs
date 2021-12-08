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
    
    characters.insert(input[0].to_string(), "1");
    characters.insert(input[1].to_string(), "7");
    characters.insert(input[2].to_string(), "4");
    characters.insert(input[9].to_string(), "8");
    
    // Find 0 and 9 by using 4
    for x in &input[6..9] {
        // Find 0 or 9 
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

        // find 6
        for letter in input[0].chars() {
            if !x.contains(letter) {
                characters.insert(x.to_string(), "6");
            }
        }
    }

    // find 2, 3 & 5
    for x in &input[3..6] {
        // find 3
        let mut condition = true;
        for letter in input[0].chars() {
            if !x.contains(letter) {
                condition = false;
            }
        }
        if condition {
            characters.insert(x.to_string(), "3");
        } else {
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
        }
    }

    let mut completed_string = "".to_string();
    for x in output {
        completed_string += characters.get(&x).unwrap();
    }
    completed_string.parse::<i32>().unwrap()
}