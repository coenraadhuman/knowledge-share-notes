_[Control Flow](./control-flow.md) << [Data Structures](./data-structures.md) >> [Standard Collections](./standard-collections.md)_

# Data Structures

## Struct (Structure)

```rust
struct Point {
    x:i32,
    y:i32
}

let p = Point { x: 1, y: 2};
println!("Point p is at ({}, {}).", p.x, p.y); // Point p is at (1, 2).
```

## Enumerations

```rust 
enum Colour {
    Red,
    Green,
    Blue,
    RgbColour(u8, u8, u8), // tuple-like construct
    CmykColour { cyan:u8, magenta:u8, yellow:u8, black:u8 } // struct-like construct
}

let c:Colour = Colour::Red;
let d:Colour = Colour::RgbColour(0, 0, 0);
let e:Colour = Colour::CmykColour{ cyan:0, magenta:128, yellow:0, black:0 };
```

## Unions

- `unsafe` block required since you assume responsibility.

```rust
// Uses 32 bits in memory.
union IntOrFloat {
    i: i32,
    f: f32
}
```

## Option<T>

```rust
let x = 3.0;
let y = 1.0;

// Option -> Some(value) | None
let mut result = if y != 0.0 { Some(x/y) } else { None };

match result {
    Some(value) => println!("{}/{} = {}", x, y, value),
    None => println!("Can't devide by zero."),
}

if let Some(value) = result { // checks whether the result can be assigned to Some(x).
    println!("if: {}/{} = {}", x, y, value);
}

while let Some(value) = result { // watch out for this, this is still a while loop.
    println!("while: {}/{} = {}", x, y, value);
    result = None;
}
```

## Arrays

```rust
let a:[i32; 5] = [1, 2, 3, 4, 5];
let b = [1, 2, 3, 4, 5];

println!("a has {} elements, first element is {}", a.len(), a[0]);
```