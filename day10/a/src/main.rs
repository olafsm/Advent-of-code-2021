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
    let mut sum = 0;

    for line in &input {
        let mut opening_brackets:Vec<char> = Vec::new();
        'inner: for c in line {
            if is_opening_bracket(*c) {
                opening_brackets.push(*c);
            } else  {
                if *c != get_matching_bracket(opening_brackets.pop().unwrap()) {
                    sum+=get_char_value(*c);
                    eprintln!("*c = {:?}", *c);
                    break 'inner;
                }
            }
        }
            
    }
    eprintln!("sum = {:?}", sum);
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

fn get_char_value(c:char) -> i32 {
    match c {
        ')' => return 3,
        ']' => return 57,
        '}' => return 1197,
        '>' => return 25137,
        _ => return 0,
    }
}