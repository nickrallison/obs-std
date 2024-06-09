---
bad_links: 
aliases: [reference count, reference counting]
tags: [coding, rust]
---
# RC Smart [[Pointer|Pointer]]

The Rust RC Smart [[Pointer|Pointer]] refers to a reference-counting smart [[Pointer|pointer]] available in the Rust programming language. 'RC' stands for 'Reference Counting'. This smart [[Pointer|pointer]] enables multiple owners for data and the data gets cleaned up once there are no more references to it. The 'RC' keeps track of the number of references to a value which determines whether or not a value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid. This is particularly useful when managing memory and preventing [[Memory Leak|memory leaks]] in concurrent programming scenarios.

```rust
use std::rc::Rc;

fn main() {
    let five = Rc::new(5);

    for _ in 0..10 {
        let _ = Rc::clone(&five);
        println!("Count after creating a new reference = {}", Rc::strong_count(&five));
    }
    
    // At this point, the count will be 11 - one for the original five, and then ten for each clone

    // Dropping the original reference
    drop(five);
    
    // After this point, the count will be 10 as we dropped one reference
}
```
In this example, an `Rc` is created with a data value of `5`. The `for` loop then clones this `Rc` ten times. Each time it is cloned, the reference count (`Rc::strong_count`) is increased by one. After creating all these references, we drop the original one which reduces the count by one. The remaining references still valid and accessible.