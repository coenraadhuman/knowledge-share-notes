_[Types and Variables](./types-and-variables.md) << [Control Flow](./control-flow.md) >> [Data Structures](./data-structures.md)_

# Control Flow

## If Statement

```rust
let temp = 25;

if temp > 30 {
    println!("Really hot outside.");
} else if temp < 10 {
    println!("Really cold outside.");
} else {
    println!("Ok temperature outside.");
}
```

The if statement is actually an expression in rust, these are still valid for nested ifs:

```rust
let day = if temp > 20 { "sunny." } else { "cloudy." };

println!("Today is {}", day); // sunny.
```

## While and Loop

```rust
let mut x = 1;

while x < 1000 {
    x *= 2;

    if x == 64 { 
        continue; // returns control flow to the beginning of the while loop. 
    };

    println!("x = {}", x);
}

let mut y = 1;

loop { // while true 
    y *= 2;
    
    if y == 64 { 
        break; // exists loop and continue with statements after loop. 
    };
}

```

## For Loops

The `continue` and `break` are still valid for for loops.

```rust
for x in 1..11 { // range which is exclusive at the end.
    println!("x = {}", x);
}

// To keep track of the position you need to use the enumerate function of a range or iterators.

for (position, val) in (30..41).enumerate() {
    println!("{}: {}", position, val)
}
```

## Match Statement

It is similar to a switch statement, but you force to cover all cases:

- Works on integers, ranges of integers, booleans, enums, tuples, arrays and structs.
- It will destructure tuples, arrays and structs.
- It requires a default handler if necessary.

```rust
let country_code = 44;

let country = match country_code {
    44 => "UK",
    1..=1000 => "Unknown", // in this case the equals ensures that the range is inclusive.
    _ => "Invalid"
};
```

## Combination Lock

This can be done using `match` and `enum`:

```rust
```