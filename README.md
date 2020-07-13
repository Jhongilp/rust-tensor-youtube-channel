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