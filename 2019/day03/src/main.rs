extern crate util;

struct Data {
    first_points: Vec<Point>,
    second_points: Vec<Point>,
}

struct Point {
    x: i32,
    y: i32,
}

fn convert_to_points(path: &str) -> Vec<Point> {
    let mut points = Vec::new();
    let mut x = 0;
    let mut y = 0;
    points.push(Point { x, y });
    for cmd in path.split(",").map(|a| a.to_string()) {
        let val = cmd.get(1..).unwrap().parse::<i32>().unwrap();
        match cmd.get(0..1) {
            Some("U") => {
                y += val;
            }
            Some("D") => {
                y -= val;
            }
            Some("L") => {
                x -= val;
            }
            Some("R") => {
                x += val;
            }

            _ => {
                println!("ups {cmd}")
            }
        }
        points.push(Point { x, y });

        // println!("{dir} / {val}");
    }
    points
}

fn get_data(filename: &str) -> Option<Data> {
    if let Some(input) = util::io::get_lines(filename) {
        // let points = convert_to_points(&input[0]);
        let data = Data {
            first_points: convert_to_points(&input[0]),
            second_points: convert_to_points(&input[1]),
        };
        Some(data)
    } else {
        None
    }
}

fn main() {
    println!("Hello, day03!");
    if let Some(data) = get_data("./03-example.data") {
        let a = 42;
        println!("{a}")
        // println!("{:?} / {:?}", data.first_points, data.first_points);
    }
}
