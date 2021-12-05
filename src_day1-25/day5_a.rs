use std::fmt;
use std::collections::HashMap;


#[derive(Clone, Hash, Eq)]
struct Point {
    x:i32,
    y:i32,
}

fn main() {
    let input:Vec<&str> = include_str!("../input copy 2.txt")
        .lines()
        .collect();

    let mut points:Vec<Point> = Vec::new();

    for line in input {
        let q:Vec<&str> = 
            line.split(" -> ").collect();
        let p1:Vec<i32> = q[0].split(",").map(|i| i.parse::<i32>().unwrap()).collect(); // x1, y1
        let p2:Vec<i32> = q[1].split(",").map(|i| i.parse::<i32>().unwrap()).collect(); // x2, y2
        
        let point1 = { Point {
            x: p1[0],
            y: p1[1],
        }};
        let point2 = { Point {
            x: p2[0],
            y: p2[1],
        }};
        if (point1.x == point1.y) && (point2.x == point2.y) {
            for i in point1.x..point2.x+1 {
                points.push(
                    Point {
                        x: i,
                        y: i,
                    }
                )
            }
        }
        if (point1.x == point2.y) && (point1.y == point2.x) {
            if point1.x < point1.y {
                let mut y = point1.y;    
                for i in point1.x..point1.y+1 {
                    points.push(
                        Point {
                            x: i,
                            y: y,
                        }
                    );
                    y -= 1;
                }
            } else if point1.x > point1.y{
                let mut x = point1.x;    
                let mut qq = point1.y;    
                for i in point1.y..point1.x+1 {
                    points.push(
                        Point {
                            x: x,
                            y: i,
                        }
                    );
                    x -= 1;
                }
            }
        } 
        else if point1.x == point2.x {
            if point1.y > point2.y {
                for i in point2.y..point1.y+1 {
                    points.push(
                        Point {
                            x:point1.x,
                            y:i,
                        }
                    );
                }
            } else {
                for i in point1.y..point2.y+1 {
                    points.push(
                        Point {
                            x:point1.x,
                            y:i,
                        }
                    );
                }

            }
        }
        else if point1.y == point2.y{
            if point1.x > point2.x {
                for i in point2.x..point1.x+1 {
                    points.push(
                        Point {
                            x:i,
                            y:point1.y,
                        }
                    )
                }
            } else {
                for i in point1.x..point2.x+1 {
                    points.push(
                        Point {
                            x:i,
                            y:point1.y,
                        }
                    )
                }

            }
        }
    }
    let mut hashes = HashMap::new();
    for point in points.clone() {
        println!("{}", point);
    }
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
