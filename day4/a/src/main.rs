/*
* Advent of Code 2021
* Day 4a
* @olafsm  
*/

fn main() {
    let input:Vec<&str> = include_str!("../input.txt")
        .split("\n\r")
        .collect();

    let mut boards:Vec<Vec<u32>> = Vec::new();
    let mut boards_marked:Vec<[bool;25]> = Vec::new();

    let numbers:Vec<u32> = input[0].trim()
        .split(",")            
        .map(|i| i.parse::<u32>().unwrap())
        .collect();

    for board in &input[1..] {
        let q:Vec<u32> = board.split_whitespace()
            .map(|i| i.parse::<u32>().unwrap())
            .collect();

        boards.push(q);
        boards_marked.push([false;25]);
    }

    'numbers_loop: for number in numbers {
        for (i, board) in boards.iter().enumerate() {
            for (j, sq) in board.iter().enumerate() {
                if number == *sq {
                    boards_marked[i][j] = true;
                }
            }
            if check_board(boards_marked[i]) {
                calculate_result(board.to_vec(), boards_marked[i], number);
                break 'numbers_loop;
            }
        }
    }
}

fn check_board(board:[bool;25]) -> bool{
    for x in board.chunks(5) {
        if x.iter().filter(|i| **i==true).count() == 5 {
            return true
        }
    }
    for n in 1..5 {
        let x:Vec<bool> = board.iter().skip(n-1).step_by(5).copied().collect();
        if x.iter().filter(|i| **i==true).count() == 5 {
            return true
        }
    }
    return false
}

fn calculate_result(board:Vec<u32>, marked_board:[bool;25], index:u32) {
    let result_sum:u32 = board.iter()
        .enumerate()
        .filter(|&(i,_)| marked_board[i] == false)
        .map(|(_, e)| e)
        .sum();
    
    println!("Calculated_result {}", result_sum * index);
}