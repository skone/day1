use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct Route {
    direction: Direction,
    distance: i32,
}

#[derive(Copy, Clone)]
pub enum Orientation {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

pub enum Direction {
    LEFT,
    RIGHT,
}

pub struct Accumulator {
    p: Point,
    o: Orientation
}

fn main() {
    let csv_route = read_file_into_string("route.csv");
    println!("{} route", csv_route);

    let v: Vec<&str> = csv_route.split(", ").collect();

    let point = Point{x: 0, y: 0};
    let orientation = Orientation::NORTH;

    let acc = Accumulator{p: point,o:orientation};

    let sum = v.into_iter().fold(acc, |xy, x| lets_walk(xy.p,xy.o,build_route(x)));

    println!("{} the sum is {} ",sum.p.x, sum.p.y);

    //for i in &v {

//        let route = build_route(i);
  //      lets_walk(point, orientation, route);

    //}

}


//Reads a file into a string and returns the result, in theb case of an error returns an empty string
pub fn read_file_into_string(file_name: &str) -> std::string::String {
    // Create a path to the desired file
    let path = Path::new(file_name);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(_) => "".to_string(),
        Ok(_) => s,
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

pub fn lets_walk(point: Point, orientation: Orientation, route: Route) -> Accumulator{

    match route.direction {
        Direction::LEFT => {
            match orientation {
                Orientation::NORTH => Accumulator{p: Point {x: (point.x - route.distance) ,y: point.y},o: Orientation::WEST},
                Orientation::EAST => Accumulator{p: Point {x: point.x ,y: point.y + route.distance},o: Orientation::NORTH},
                Orientation::SOUTH => Accumulator{p: Point {x: (point.x + route.distance) ,y: point.y},o: Orientation::EAST},
                Orientation::WEST => Accumulator{p: Point {x: point.x ,y: point.y - route.distance},o: Orientation::SOUTH},
            }
        },
        Direction::RIGHT => {
            match orientation {
                Orientation::NORTH => Accumulator{p: Point {x: (point.x + route.distance) ,y: point.y},o: Orientation::EAST},
                Orientation::EAST => Accumulator{p: Point {x: point.x ,y: (point.y - route.distance)},o: Orientation::SOUTH},
                Orientation::SOUTH => Accumulator{p: Point {x: (point.x - route.distance) ,y: point.y},o: Orientation::WEST},
                Orientation::WEST => Accumulator{p: Point {x: point.x ,y: (point.y + route.distance)},o: Orientation::NORTH},
            }

        },
    }

}

pub fn build_route(string_route: &str) -> Route{
    Route { direction: build_direction(string_route), distance: get_distance(string_route)}
}

pub fn build_direction(string_route: &str) -> Direction{
    let first_char_option = string_route.chars().next();


    //This isn't quite right, probably need to panic if no match find for either condition
    match first_char_option {
        Some(x) => {       match x {
                      'R' => Direction::RIGHT,
                        _ => Direction::LEFT
                   }},
        None => {Direction::RIGHT}
    }
}

pub fn get_distance(string_route: &str) -> i32 {


    let result = string_route.trim_left_matches(|c: char| !c.is_numeric());
    let parsed_result = result.to_string().parse::<i32>();

    match parsed_result {
        Ok(x) => { x },
        Err(e) => {println!("Error parsing {}", e); 0}
    }
}
