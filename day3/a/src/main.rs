/*
* Advent of Code 2021
* Day 3a
* @olafsm  
*/

fn main() {
    let input:Vec<&str> = include_str!("../input.txt")
        .lines()
        .collect();
    let len = input[0].len();
    let mut gamma_rate = String::from("------------");
    let mut epsilon_rate = String::from("------------");
    for i in 0..len {
        let mut one = 0;
        let mut zero = 0;
        for line in &input {
            match line[i..i+1].as_ref() {
                "1" => one+=1,
                "0" => zero+=1,
                _ => {},
            }
        }
        if one>zero {
            gamma_rate.replace_range(i..i+1, "1"); 
            epsilon_rate.replace_range(i..i+1, "0"); 
        } else {
            gamma_rate.replace_range(i..i+1, "0"); 
            epsilon_rate.replace_range(i..i+1, "1"); 
        }
    }
    let gamma_decimal = isize::from_str_radix(&gamma_rate, 2).unwrap();
    println!("Gamma rate binary: {}", gamma_rate);
    println!("Gamma rate decimal: {}", gamma_decimal);

    let epsilon_decimal = isize::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("Epsilon rate binary: {}", epsilon_rate);
    println!("Epsilon rate decimal: {}", epsilon_decimal);

    println!("Decimal product: {}", epsilon_decimal*gamma_decimal);
}