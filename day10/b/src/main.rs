/*
* Advent of Code 2021
* Day 10a
* @olafsm  
*/
fn main() {
    let input:Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map({ |x|
            x.chars().collect()
        })
        .collect();

    let mut scores:Vec<u64> = Vec::new();

    'outer: for line in &input {
        let mut opening_brackets:Vec<char> = Vec::new();
        for c in line {
            if is_opening_bracket(*c) {
                opening_brackets.push(*c);
            }
            else if *c != get_matching_bracket(opening_brackets.pop().unwrap()) {
                continue 'outer;
            }
        }
        let mut score:u64 = 0;
        for c in opening_brackets.iter().rev() {
            score *= 5;
            score += get_char_value(get_matching_bracket(*c));
        }
        scores.push(score);
    }
    scores.sort();
    let middle_index = scores.len()/2;
    eprintln!("sum = {:?}", scores[middle_index]);
}   

fn is_opening_bracket(c:char) -> bool{
    ['(', '[', '{', '<'].contains(&c)
}
fn get_matching_bracket(c:char) -> char {
    match c {
        '(' => return ')',
        '[' => return ']',
        '{' => return '}',
        '<' => return '>',
        _ => return 'q',
    }
}
//fn is_closing_bracket(c:char) -> bool{
//    [')', ']', '}', '>'].contains(&c)
//}

fn get_char_value(c:char) -> u64 {
    match c {
        ')' => return 1,
        ']' => return 2,
        '}' => return 3,
        '>' => return 4,
        _ => return 0,
    }
}