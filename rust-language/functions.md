_[Characters and Strings](./text.md) << [Functions](./functions.md) >> [Traits](./traits.md)_

# Functions

## Functions and Function Arguments

`Parameters`, refer to the special variables that make up an function's signature.

`Arguments`, refer to the values that are provided to the function's parameters.

:nerd: People tend to use these two terms interchangeably.

```rust
fn print_value(x: i32) { // argument goes onto the stack.
    println!("Value = {}.")
}

fn increase(x: &mut i32) { // reference of data that is mutable.
    *x += 1; // 
}

fn product(x: i32, y: i32) -> i32 { // specifies return type.
    x * y // to return a value in rust, you should not add a ';' which would make it a statement.
}

fn main() {
    print_value(33);
    let mut z = 1;    
    increase(&mut z);
}
```

## Methods

A function for a stucture (struct) is called a method.

```rust 
struct Point {
    x: f64,
    y: f64
}

struct Line { // defining the struct itself and its variables.
    start: Point,
    end: Point
}

impl Line { // defining the methods for the struct.
    fn len(&self) -> f64 { // we are passing the reference of the struct itself.
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        
        (dx*dx + dy*dy).sqrt() // no semicolon, so this is the return value.
    }
}


```