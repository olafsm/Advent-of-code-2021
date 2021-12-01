fn main() {
    let input = include_str!("../input.txt");
    let mut x = 0;
    let mut prev = -777;
    for line in input.lines() {
        let cur = line.parse::<i32>().unwrap();

        if (prev != -777) & (prev < cur) {
            x += 1;
        }
        prev = cur;
    }
    println!("{}", x);
}
