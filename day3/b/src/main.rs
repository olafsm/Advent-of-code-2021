/*
* Advent of Code 2021
* Day 3b
* @olafsm  
*/

fn main() {
    let input:Vec<&str> = include_str!("../input.txt")
        .lines()
        .collect();
    
    let oxygen_rating = get_rating(input.clone(), "oxygen");
    println!("Oxygen rating: {}", oxygen_rating);
    
    
    let co2_rating = get_rating(input.clone(), "co2");
    println!("Co2 rating: {}", co2_rating);

    println!("Product: {}", co2_rating * oxygen_rating);
}

fn get_rating(mut numbers:Vec<&str>, rating_type: &str) -> isize {
    for i in 0..12 {
        if numbers.len() > 1 {
            let mut one = 0;
            let mut zero = 0;    
            for line in &numbers {
                match line[i..i+1].as_ref() {
                    "1" => one+=1,
                    "0" => zero+=1,
                    _ => {},
                }
            }

            let character = 
                if (rating_type == "oxygen" && one < zero ) 
                    | (rating_type == "co2" && one >= zero )
                {'0'}
                else {'1'};

            let mut j = 0;
            while j < numbers.len() {
                if numbers[j].chars().nth(i).unwrap() != character {
                    numbers.remove(j);
                } else {
                    j += 1;
                }
            }    
        }
    }
    let rating = isize::from_str_radix(&numbers[0], 2).unwrap();
    rating
}