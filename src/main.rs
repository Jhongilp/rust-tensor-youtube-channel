#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    End,
}

use List::Cons;
use List::End;

fn main() {
    let b = Box::new(10);
    println!("B {}", b);

    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));
    println!("l: {:?}", l);
}