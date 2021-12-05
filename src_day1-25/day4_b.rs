/*
* Advent of Code 2021
* Day 4b
* @olafsm  
*/

fn main() {
    let input:Vec<&str> = include_str!("../input.txt")
        .split("\n\r")
        .collect();

    let mut boards:Vec<Vec<u32>> = Vec::new();
    let mut boards_marked:Vec<[bool;25]> = Vec::new();
    let mut boards_won:Vec<usize> = Vec::new();
    let mut last_won_number = 0;

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

    for number in numbers {
        for (i, board) in boards.iter().enumerate() {
            if boards_won.contains(&i) {
                continue;
            }
            for (j, sq) in board.iter().enumerate() {
                if number == *sq {
                    boards_marked[i][j] = true;
                }
            }
            if check_board(boards_marked[i]) {
                boards_won.push(i);
                last_won_number = number;
            }
        }
    }

    let last_won_index = boards_won[boards_won.len()-1];
    calculate_result(boards.remove(last_won_index), boards_marked[last_won_index], last_won_number);
}

fn check_board(board:[bool;25]) -> bool{
    for x in board.chunks(5) {
        if x.iter().filter(|i| **i==true).count() == 5 {
            return true
        }
    }
    for n in 1..6 {
        let x:Vec<bool> = board.iter().skip(n-1).step_by(5).copied().collect();
        if x.iter().filter(|i| **i==true).count() == 5 {
            return true
        }
    }
    return false
}

fn calculate_result(board:Vec<u32>, marked_board:[bool;25], last_number:u32) {
    let result_sum:u32 = board.iter()
        .enumerate()
        .filter(|&(i,_)| marked_board[i] == false)
        .map(|(_, e)| e)
        .sum();

    println!("Result sum {}", result_sum);
    println!("last_won_number {}", last_number);
    println!("Calculated_result {}", result_sum * last_number);
}