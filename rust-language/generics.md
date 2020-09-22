_[Pattern Matching](./pattern-matching.md) << Generics >> [Standard Collections](./standard-collections.md)_

# Generics

Generics in a nutshell.

```rust
struct Point<T> {
    x: T,
    y: T
}

fn generics() {
    let a = Point { x: 0, y: 4 }; // Point<i32> 
    let b = Point { x: 0.0, y: 4f64 }; // Point<f64>

    let c:Point<u16> = Point { x: 0, y: 4 }; 
    let b:Point<f32> = Point { x: 0.0, y: 4.0 }; 
}
```