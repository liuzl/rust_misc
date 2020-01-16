#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = "RUC";
    let age = 81;
    let ruc = Person {name, age};

    println!("{:?}", ruc);

    let point: Point = Point {x:10.3, y:0.4};
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point {x:5.2, ..point};
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {x: abc, y: xyz} = point;
    let _rec = Rectangle {
        top_left: Point {x: abc, y: xyz},
        bottom_right: bottom_right,
    };

    let _nil = Nil;
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}
