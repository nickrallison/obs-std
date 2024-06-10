---
bad_links: 
aliases: []
tags: [coding, rust]
---
# Static vs. Dynamic Dispatch

Static and dynamic dispatch are two methods that a programming language like Rust uses to decide which function or method to call at runtime. 

Static dispatch happens at compile time, where the exact function or method to be called is known and directly inserted into the code. This can lead to faster execution times and better optimization, as the compiler has full knowledge of the function being called. However, it can also result in larger binary sizes due to duplication of code.

On the other hand, dynamic dispatch happens at runtime. The program determines which function to call based on [[Information Theory|information]] available while it's executing. This allows for more flexibility and polymorphism as different functions can be called based on the state of the program. However, it may have a slight performance overhead due to the need for indirection (i.e., looking up which function to call).

In Rust, static dispatch is achieved through monomorphization (creating separate functions for each type at compile time) using generics and [[Trait Bounds|trait bounds]]. Dynamic dispatch is done using [[Rust Traits|trait]] objects that allow for multiple types of objects to be handled in a similar way at runtime.

Static Dispatch example:
```rust
[[Traits.md|trait]] Animal {
    fn sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn sound(&self) {
        println!("Dog says Woof!");
    }
}

impl Animal for Cat {
    fn sound(&self) {
        println!("Cat says Meow!");
    }
}

fn make_sound<T: Animal>(animal: T) {
    animal.sound();
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    make_sound(dog);
    make_sound(cat);
}
```
In the above Rust code, we have a [[Rust Traits|trait]] `Animal` with one method `sound()`. Two structs `Dog` and `Cat` implement this [[Rust Traits|trait]]. The function `make_sound()` uses static dispatch to call the appropriate implementation of the `sound()` method at compile time.

Dynamic Dispatch example:
```rust
[[Traits.md|trait]] Animal {
    fn sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn sound(&self) {
        println!("Dog says Woof!");
    }
}

impl Animal for Cat {
    fn sound(&self) {
        println!("Cat says Meow!");
    }
}

fn make_sound(animal: &dyn Animal) {  
  animal.sound();  
}  

fn main() {  
  let dog = Dog;  
  let cat = Cat;  

  make_sound(&dog);  
  make_sound(&cat);  
}
```
In the above code, the function `make_sound()` takes a reference to a [[Rust Traits|trait]] object (`&dyn Animal`). This allows it to accept any type that implements the `Animal` [[Rust Traits|trait]]. The appropriate implementation of the `sound()` method is determined at runtime.
