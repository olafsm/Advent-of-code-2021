/*
* Advent of Code 2021
* Day 9b
* @olafsm  
*/
use std::time::Instant;
fn main() {
    let before = Instant::now();
    let mut input:Vec<Vec<i32>> = include_str!("../input copy 2.txt")
        .lines()
        .map(|x| {
            x.chars().map( |y| {
                y.to_digit(10).unwrap() as i32
            }).collect()
        }).collect();
    let input_length = input.len();
    let input_width = input[0].len();
    let mut visited_input = vec![vec![false;input_width];input_length];


    let mut basins:Vec<u64> = Vec::new();
    let mut basin_points:Vec<Vec<(usize,usize)>> = Vec::new();

    for (i, line) in input.iter().enumerate() {
        for (j, num) in line.iter().enumerate() {
            
            if visited_input[i][j] || *num == 9 { continue; }
            
            visited_input[i][j] = true;
            
            let mut basin_size = 0;
            let mut temp_basin_points:Vec<(usize, usize)> = Vec::new();
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
                        temp_basin_points.push((k-1, l));
                    }
                }
                if k+1 < input.len()  {
                    if !visited_input[k+1][l] && (input[k+1][l] != 9) {
                        visited_input[k+1][l] = true;
                        q.push((k+1, l));
                        temp_basin_points.push((k+1, l));
                    }
                }
                if l > 0  {
                    if! visited_input[k][l-1] && (input[k][l-1] != 9) {
                        visited_input[k][l-1] = true;
                        q.push((k, l-1));
                        temp_basin_points.push((k, l-1));
                    }
                }
                if l+1 < line.len()  {
                    if !visited_input[k][l+1] && (input[k][l+1] != 9) {
                        visited_input[k][l+1] = true;
                        q.push((k, l+1));
                        temp_basin_points.push((k, l+1));
                    }
                }
            }
            basins.push(basin_size as u64);
            basin_points.push(temp_basin_points);
        }
    }

    // Visualization
    //basin_points.sort_by(|a, b| a.len().cmp(&b.len()));
    //for basin in basin_points[basin_points.len()-3..].iter() {
    //    for (x,y) in basin {
    //        input[*x][*y] = 777;
    //    }
    //}
    basins.sort();
    //for line in input {
    //    for pos in line {
    //        if pos == 777{
    //            print!("0");
    //        } else if pos != 9 {
    //            print!("-");
    //        }
    //        else {
    //            print!("9");
    //        }
    //    }
    //    print!("\n");
    //}

    // Result
    let total:u64 = basins[basins.len()-3..].iter().product();
    println!("Elapsed time: {:.2?}", before.elapsed());
    eprintln!("total = {:?}", total);
}