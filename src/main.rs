#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direccion {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direccion {
    fn match_direction(&self) -> Keys {
        match self {
            Direccion::Up(_) => Keys::UpKey(String::from("Pressed W")),
            Direccion::Down(_) => Keys::DownKey(String::from("Pressed W")),
            Direccion::Left(_) => Keys::LeftKey(String::from("Pressed W")),
            Direccion::Right(_) => Keys::RightKey(String::from("Pressed W")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match self {
            Keys::UpKey(s) => s,
            Keys::DownKey(s) => s,
            Keys::LeftKey(s) => s,
            Keys::RightKey(s) => s,
        }
    }
}


enum Shape {
    Rectangle {width: u32, height: u32},
    Square(u32),
    Circle(f64),
}


impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle {width, height} => (width * height) as f64,
            Shape::Square(s) => (s * s) as f64,
            Shape::Circle(r) => 3.14 * (r * r),
        }
    }
}

fn main() {
    let u = Direccion::Up(Point { x: 0, y: 10 });

    let v = Direccion::Down(Point { x: 20, y: 30 });

    let up = u.match_direction();
    let up_s = up.destruct();
    println!("{:?}", up);
    println!("{:?}", up_s);

    let r = Shape::Rectangle{width: 10, height:70};
    let s = Shape::Square(20);
    let c = Shape::Circle(50.0);

    let ar = r.area();
    print!("rectangle area: {}", ar);
}
