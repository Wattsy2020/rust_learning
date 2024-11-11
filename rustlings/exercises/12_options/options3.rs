#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    // adding a reference to the optional also works
    // but in the None case there's no need to have that be a reference
    // so we can just take a reference to the data in the some case using ref
    match optional_point {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        None => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
