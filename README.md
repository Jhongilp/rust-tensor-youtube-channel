# Learning Rust

Learning Rust using Tensor Programming [Youtube Channel](https://www.youtube.com/watch?v=EYqceb2AnkU&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW)

- [enums and options](https://www.youtube.com/watch?v=-E2qL4bLDKo&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW&index=6)

## enums

enums can contain any type of data:

```
enum Direccion {
    Up(u32, i32, String), // tupple
    Down {x: u32, y: f64}, // struct
    Left, // union
    Right
}
```

Example of enums of structs:

```
enum Direccion {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let u = Direccion::Up(Point { x: 0, y: 10 });

    let v = Direccion::Down(Point { x: 20, y: 30 });
}
```

enums can also have implementations:

```
impl Direccion {
    fn match_direction(&self) -> Keys {
        match *self {
            Direccion::Up(_) => Keys::UpKey(String::from("Pressed W")),
            Direccion::Down(_) => Keys::DownKey(String::from("Pressed W")),
            Direccion::Left(_) => Keys::LeftKey(String::from("Pressed W")),
            Direccion::Right(_) => Keys::RightKey(String::from("Pressed W")),
        }
    }
}
```

You can destruct the enum type:

```
impl Keys {
    fn destruct(&self) -> &String {
        match self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}
```

More example of enum using strucs and impl:

```
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
    let r = Shape::Rectangle{width: 10, height:70};
    let s = Shape::Square(20);
    let c = Shape::Circle(50.0);

    let ar = r.area();
    print!("rectangle area: {}", ar);
}
```

## Option

There is not null value in Rust, instead we use Option:

```
fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    // let res = division(5.0, 0.0);
    let res = division(5.0, 0.1125);
    match res {
        Some(x) => println!("division result: {:.2}", x), // two decimals
        None => println!("cannot divide by zero"),
    }
}
```