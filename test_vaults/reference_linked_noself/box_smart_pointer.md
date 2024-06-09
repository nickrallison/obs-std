---
bad_links: 
aliases: []
tags: [coding, rust]
---
# Box Smart [[Pointer|Pointer]]

The Rust Box smart [[Pointer|pointer]] is a non-cloneable smart [[Pointer|pointer]] type provided by the Rust programming language. It is used for allocating values on the [[Heap Data Structure|heap]] and allows for transferring ownership between functions. It also ensures that the allocated memory is cleaned up appropriately when it's no longer needed. The Box smart [[Pointer|pointer]] can be used to handle large amounts of data or transfer data between different parts of the program, where copying the data would be too expensive in terms of performance.

```rust
// Defining a function that takes ownership of a box
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed here, and the memory freed
}

fn main() {
    // Stack allocated integer
    let x = 5u32;

    // Copy the data contained in `x` to `y`
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a [[Pointer.md|pointer]] to a [[Heap Data Structure.md|heap]] allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // Transfer ownership of `a` to `b`
    let b = a;
    
    // Error! `a` no longer has access to the data because it no longer owns the [[Heap Data Structure.md|heap]] memory
    //println!("a contains: {}", a); 

     // This function takes ownership of the [[Heap Data Structure.md|heap]] allocated memory from `b`
     destroy_box(b);
}
```

In this example, we have an integer `x` on stack. We copy its value into another integer `y`. Then we create an integer on the [[Heap Data Structure|heap]] using Rust's Box smart [[Pointer|pointer]]. We then try to transfer the ownership from variable 'a' (which initially owns the Box) to 'b'. After this transfer of ownership, if we try to use 'a' it will result in error as 'a' no longer has access to that memory.  
Finally, we pass 'b' (which now owns the Box) into function destroy_box which will clean up and free up our [[Heap Data Structure|heap]] allocated memory.