enum List {
    Cons(i32, List),
    End,
}

fn main() {
    let b = Box::new(10);
    println!("B {}", b);


}