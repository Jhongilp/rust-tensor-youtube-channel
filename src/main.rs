trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.141 * self.radius * self.radius) as u32
    }
}

// ++++++++

struct Fib {
    c: u32,
    n: u32,
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        Some(self.c)
    }
}

fn fib() -> Fib {
    Fib {c: 1, n: 1}
}


fn main() {
    for j in fib().take(5) {
        println!("{}", j);
    }

    for j in fib().skip(14).take(10) {
        println!("{}", j)
    }

    let mut f = fib();

    println!("next: {:?}", f.next());
    println!("next: {:?}", f.next());
    println!("next: {:?}", f.next());
    println!("next: {:?}", f.next());
    println!("next: {:?}", f.next());
}
