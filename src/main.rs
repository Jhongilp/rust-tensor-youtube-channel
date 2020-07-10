
fn main() {
    let v = vec![1,2,5,6,2,1,5,2,6,2,2,8,8,5,6,9,4,5,4,1,2,4,1,2,3,4,5,6,7,8,9];

    for &i in &v {
        let r = count(&v, i);
        println!("{} is repeated {} times", i, r);
    }

}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.iter().filter(|&&x| x == val).count()
    // v.iter().filter(|x| **x == val).count() //this is the same as above
}




// fn main() {
    
//     let mut v = Vec::new();

//     for i in 0..1000 {
//         v.push(i);
//     }

//     println!("main still owns v: {}", v[200]);
//     v = return_what_borrow(v);

//     let y = &mut v;

//     println!("main still owns v: {}", y[200]);
//     y[200] = 666;
//     println!("main y points to v: {}", y[200]);

//     borrow_by_reference(y);

    
// }

// fn return_what_borrow(mut v: Vec<i32>) -> Vec<i32> {
//     v[200] = 333;
//     println!("vec borrowed: {}", v[200]);
//     v
// }

// fn borrow_by_reference(v: &Vec<i32>) {
//     println!("vec borrowed by reference: {}", v[200]);
// }
