use std::fmt;
use std::collections::HashMap;


#[derive(Clone, Hash, Eq)]
struct Point {
    x:i32,
    y:i32,
}

fn main() {
    let input:Vec<&str> = include_str!("../input.txt")
        .lines()
        .collect();

    let mut points:Vec<Point> = Vec::new();

    for line in input {
        let q:Vec<&str> = 
            line.split(" -> ").collect();
            
        let point1:Vec<i32> = q[0].split(",").map(|i| i.parse::<i32>().unwrap()).collect(); // x1, y1
        let point2:Vec<i32> = q[1].split(",").map(|i| i.parse::<i32>().unwrap()).collect(); // x2, y2
        
        let x1 = point1[0];
        let x2 = point2[0];

        let y1 = point1[1];
        let y2 = point2[1];

        // 8,0 -> 8,1
        if (x1 == x2) && (y1 != y2) {
            if y1<=y2 {
                for i in y1..y2+1 {
                    points.push(Point{x:x1,y:i})
                }
            }else {
                for i in y2..y1+1 {
                    points.push(Point{x:x1,y:i})
                }
            }
        } 
        // 0,8 -> 1,8
        else if (y1 == y2) && (x1 != x2) {
            if x1<=x2 {
                for i in x1..x2+1 {
                    points.push(Point{x:i,y:y1});
                }
            }else {
                for i in x2..x1+1 {
                    points.push(Point{x:i,y:y1});
                }
            }
        } 
        else {
            if x1<x2 {
                // 0,0  8,8
                if y1<y2 {
                    let mut y = y1;
                    for x in x1..x2+1 {
                        points.push(Point{x,y});
                        y+=1;
                    }
                }
                // 0,8 8,0
                else {
                    let mut y = y1;
                    for x in x1..x2+1 {
                        points.push(Point{x,y});
                        y-=1;
                    }
                }
            } 
            else if x1>x2 {
                // 8,0  0,8
                if y1<y2 {
                    let mut x = x1;
                    for y in y1..y2+1 {
                        points.push(Point{x,y});
                        x-=1;
                    }
                }
                // 8,8  0,0
                else {
                    let mut y = y1;
                    let mut x = x1;
                    while y != y2-1 {
                        points.push(Point{x,y});
                        y-=1;
                        x-=1;
                    }
                }
            }
        }
    }
    let mut hashes = HashMap::new();
    for point in points {
        if hashes.contains_key(&point) {
            *hashes.get_mut(&point).unwrap()+=1;
            continue;
        }
        hashes.insert(point, 1);
    }
    let res = hashes.iter().filter(|i| i.1 >= &2).count();
    println!("{}", res);
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other:&Self ) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
} 
