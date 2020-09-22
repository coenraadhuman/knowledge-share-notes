_[Data Structures](./data-structures.md) << Pattern Matching >> [Generics](./generics.md)_

# Pattern Matching

You could either pass by value or reference.

// todo: add colour example with numerous values in stuct and example of pass by reference.

```rust
fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        9...11 => "lots of", // inclusive range
        12 => "a dozen",
        _ if (x % 2 == 0) => "some", // another condition for the match
        _ => "a few"
    }
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x))
    }

    let point = (3, 4);
    
    match (point) {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (x, y) => println!("({}, {})", x, y) // (_, y) => println!("(?, {})", y) , we don't care about the value of x
    }
}
```