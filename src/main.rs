
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

fn main() {
    let u = Direccion::Up(Point { x: 0, y: 10 });

    let v = Direccion::Down(Point { x: 20, y: 30 });

    // let up = Direccion::Up(Point {x: 0, y: 0}).match_direction();
    let up = u.match_direction();
    println!("{:?}", up);
}
