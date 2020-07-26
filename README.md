# Learning Rust

Learning Rust using Tensor Programming [Youtube Channel](https://www.youtube.com/watch?v=EYqceb2AnkU&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW)


## Smart pointers (Box Pointer)

Rust book reference: [15.1 Using Box to Point to data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)

> Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.

You’ll use them most often in these situations:

- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type


### When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size

