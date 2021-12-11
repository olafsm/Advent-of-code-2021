use std::vec;

fn main() {
    let mut input:Vec<Vec<u8>> = include_str!("../input.txt")
        .lines()
        .map(|line|
            line.chars()
                .map(|c| c as u8 - 0x30)
                .collect()
        )
        .collect();
    
    let adjacent = [
        (0,1),(0,-1), 
        (-1,1),(-1,0),(-1,-1),
        (1,1),(1,0),(1,-1)
    ];
    let mut steps = 0;
    let mut all_flashed = false;
    
    while !all_flashed {
        let mut input_clone:Vec<Vec<u8>> = input.iter().map(|line| {
            line.iter().map(|x|x+1).collect()
        }).collect();

        let mut flashed:Vec<Vec<bool>> = vec![vec![false;input[0].len()];input.len()];
        let mut cont = true;

        while cont {
            cont = false;
            for (x,line) in input.iter().enumerate() {
                for (y, o) in line.iter().enumerate() {
                    if input_clone[x][y] < 10 || flashed[x][y] == true {
                        continue;
                    }
                    flashed[x][y] = true;
                    for (dx, dy) in adjacent {
                        if (dx == -1 && x == 0) || (dy == -1 && y == 0) 
                            || (dx == 1 && x == input[0].len()-1) || (dy == 1 && y == input.len()-1)
                        {
                            continue;
                        }
                        let nx = (x as i32 + dx as i32) as usize;
                        let ny = (y as i32 + dy as i32) as usize;
                        //eprintln!("{},{} -> {:?}, {:?}", x, y, dx, dy);
                        input_clone[nx][ny] += 1;
                        if input_clone[nx][ny] >= 9 {
                            cont = true;
                        }
                    }
                }
            }
        }
        let mut temp_flashes = 0;
        for (i, line) in flashed.iter().enumerate() {
            for(j, b) in line.iter().enumerate(){
                if b == &true {
                    input_clone[i][j] = 0;
                    temp_flashes += 1;
                }
            }
        }
        if temp_flashes == input[0].len()*input.len() {
            all_flashed = true;
            eprintln!("index = {:?}", steps);
            print_2d(&input_clone);    
        }
        steps += 1;
        input = input_clone;
    }
}

fn print_2d(a:&Vec<Vec<u8>>) {
    for line in a {
        for c in line {
            print!("{}",c);
        }
        print!("\n");
    }
}