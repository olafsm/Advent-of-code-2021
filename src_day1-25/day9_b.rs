/*
* Advent of Code 2021
* Day 9b
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
    let input_length = input.len();
    let input_width = input[0].len();
    let mut visited_input = vec![vec![false;input_width];input_length];


    let mut basins:Vec<i32> = Vec::new();

    for (i, line) in input.iter().enumerate() {
        for (j, num) in line.iter().enumerate() {
            
            if visited_input[i][j] || *num == 9 { continue; }
            
            visited_input[i][j] = true;
            
            let mut basin_size = 0;
            let mut q:Vec<(usize, usize)> = Vec::new();
            q.push((i,j));
            while !q.is_empty() {
                let (k,l) = q.pop().unwrap();

                visited_input[k][l] = true;
                basin_size += 1;
                if k > 0 {
                    if !visited_input[k-1][l] && (input[k-1][l] != 9) {
                        visited_input[k-1][l] = true;
                        q.push((k-1, l));
                    }
                }
                if k+1 < input.len()  {
                    if !visited_input[k+1][l] && (input[k+1][l] != 9) {
                        visited_input[k+1][l] = true;
                        q.push((k+1, l));
                    }
                }
                if l > 0  {
                    if! visited_input[k][l-1] && (input[k][l-1] != 9) {
                        visited_input[k][l-1] = true;
                        q.push((k, l-1));
                    }
                }
                if l+1 < line.len()  {
                    if !visited_input[k][l+1] && (input[k][l+1] != 9) {
                        visited_input[k][l+1] = true;
                        q.push((k, l+1));
                    }
                }
            }
            basins.push(basin_size);
        }
    }
    basins.sort();
    let total:i32 = basins[basins.len()-3..].iter().product();
    eprintln!("total = {:?}", total);
}