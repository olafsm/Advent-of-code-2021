/*
* Advent of Code 2021
* Day 7b
* @olafsm  
*/

fn main() {
    let input:Vec<i32> = include_str!("../input.txt")
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut min_sum = i32::MAX;
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    
    for i in *min..*max {
        let cost = get_fuel_cost(input.clone(), i);
        if cost < min_sum {
            min_sum = cost;
        }
    }
    println!("{:?}", min_sum);
    println!("{:?}", min);
    println!("{:?}", max);
}

fn get_fuel_cost(input:Vec<i32>, num:i32) -> i32 {
    input.iter() 
        .map(|x| ((x-num).abs()*((x-num).abs()+1))/2)
        .sum()
}
