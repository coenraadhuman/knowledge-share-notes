_[Generics](./generics.md) << Standard Collections >> [Characters and Strings](./characters-strings.md)_

# Standard Collections

Overview of collections from [The Rust Programming Language Udemy Course](https://www.udemy.com/course/rust-lang/)

![Overview of collections](./images/1-standard-collections.png)

## Vec(tor)

Basically a list.

```rust
fn main() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);   
    a.push(3);   

    // usize & isize
    let index:usize = 0; // the datatype here is important, since it has to be the size of the memory of the system and can't be negative, thus unsigned.

    println!("{}", a[index]);

    // safe way of getting a element:

    match a.get(90) { // returns a option.
        Some() => println!("{}", x),
        None => println!("Element does not exist.")
    }

    for x in &a { // note the use of the reference and not the value.
        println!("{}", x);
    }   

    let last_element = a.pop(); // returns a option.

    while let Some(x) = a.pop() { // assigns and evaluates the expression.
        println!("{}", a);
    }
}
```

## Hashmap

```rust
fn main() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle", 3));
    println!("{}", shapes["triangle"].into());
    shapes.entry("circle".into()).or_insert(1);
}
```

## Hashset

```rust
fn main() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    greeks.insert("gamma");
    greeks.insert("gamma");
    greeks.insert("gamma"); // returns boolean indicating a successful insert.
    println!("{:?}", greeks); // {"delta", "gamma"}
    if !greeks.contains("kappa") {
        println!("We don't have kappa!"); 
    }
    greeks.remove("delta"); // returns boolean indicating a successful remove.

    // mathematical set operations:
    let a: HashSet<_> = (1..=5).collect();
    let b: HashSet<_> = (6..=10).collect();
    let c: HashSet<_> = (1..=10).collect();
    let d: HashSet<_> = (2..=8).collect();
    
    // subset
    println!("Is {:?} a subset of {:?}? {}", d, c, d.is_subset(&c)); // true

    // disjoint
    println!("Is {:?} a subset of {:?}? {}", a, b, a.is_disjoint(&b)); // true

    // union
    println!("Is {:?} a subset of {:?}? {}", d, b, d.union(&b)); // returns set with them.    
}
```

## Iterators

Moves note: `In Rust, for most types, operations like assigning a value to a variable, passing it to a function, or returning it from a function don't copy the value: they move it. The source relinquishes ownership of the value to the destination, and becomes uninitialized; the destination now controls the value's lifetime.` - [Ownership - Programming Rust by O'Reilly](https://www.oreilly.com/library/view/programming-rust/9781491927274/ch04.html#:~:text=the%20next%20chapter.-,Moves,now%20controls%20the%20value's%20lifetime.)

```rust
fn main() {
    let vec = vec![3, 2, 1]; // macro for creating vector.
    for x in &vec { // borrow it, can't use 'vec' since it moves that vec.
        println!("{}", x);
    }

    for x in vec.iter() { // by reference and immutable, iter_mut() allows for mutable reference.
        println!("{}", *x);
    }

    for x in vec.iter().rev() { // reverse order.
        println!("{}", *x);
    }

    // move operation on vectors
    let mut vec = vec![3, 2, 1];
    let mut vec2 = vec![1, 2, 3];
    vec2.extend(vec); // moves values from vec one to two, this operation uses the vec.into_into_iter(); which transforms it into a iterator.
    println!("{:?}", vec2);
    
}
```
