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

    // using Box as reference
    let y = 4;
    let x = &y;
    let z = Box::new(y);

    if *x == *z {
        println!("x a z has the same value: {} = {}", *x, *z);
    }

    let address_y = format!("address y: {:p}", x);
    let address_z = format!("address z: {:p}", z);
    println!("{}", address_y);
    println!("{}", address_z);
}