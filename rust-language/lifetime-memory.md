_[Traits](./traits.md) << Lifetime and Memory >> [Advanced Topics]()_

# Lifetime and Memory

## Ownership

```rust
fn main() {
    // concept that only one variable owns a specific data in memory.
    let v = vec![1, 2, 3]; // v owns the vector and the v is on the stack and the data is on the heap.
    
    // rust memory conditions are violated since this could introduce a race condition.
    let v2 = v; // `assigns a new pointer to the same data`, but in Rust this has actually been moved.

    // v is essentially dead.

    let u = 1; 
    let u2 = u; // this is valid, the reason being that it copies the value. If it was a boxed value this would not apply, but the above.
    
    println("{}", u);

    // so what you need to do is return the value so that you return ownership.
    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };
    
    let v2 = print_vector(v2); // really annoying, so the concept of borrowing exists.

}
```

## Borrowing

```rust
fn main() {
     let print_vector = |x:&Vec<i32>| -> Vec<i32> {
        println!("{:?}", *x);
        x
     };
    
    let v = vec![1, 2, 3];

    print_vector(&v); // we give it the reference of the vector and it borrows for a little while.

    println!("{:?}", v); // this is now legal.

    // mutable -> only one variable should be allowed to change data to avoid race

    let mut a = 40;

    let b = &mut a;

    *b += 2; // the star allows you to access the data the reference is pointing to.

    println!("{}", a); // this is illegal, since we need to stop the borrowing. One way is to introduce lifetime scope with curly braces.

    let mut c = 40;
    {
       let d = &mut c;
           
       *d += 2; 
    }  // d stops borrow c's data.
    
    println!("{}", c); // legal
}
```

## Lifetime

```rust
struct Person {
    name: String
}

impl Person {
    // fn get_ref_name(&self) -> &String {
    fn get_ref_name<'a>(&'a self) -> &'a String { // lifetime elision, done by compiler.
        &self.name
    }
}

// struct Company {
struct Company<'z> { // indicates the lifetime
    name: String,
    // ceo: &Person
    ceo: &'z Person // indicates the lifetime
}

fn main() {
    // &'static str // static lifetime = lifetime of program.

    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss };

    // the above results in missing lifetime error, why: at any point boss could be invalid and Rust is against it.
    // to make this valid you need to provide guarantees that both would be have same lifetime.
}
```

## Lifetime in Structure Implementation

```rust
struct Person<'a> { // this person can't outlive the name, hence the lifetime.
    name: &'a str // reference to another string with same lifetime.
}

impl<'a> Person<'a> { // for the implementation we need to specify a lifetime as well.
    fn talk(&self) {
        println!("Hi, my name is {}.", *self.name);    
    }   
}

fn main() {
    let person = Person{ name: "Coenraad" };
}
```

## Reference-Counted Variables (Rc)

```rust
use std::rc::Rc; // A single-threaded reference-counting pointer. 'Rc' stands for 'Reference Counted'. 
                 // Create a lifetime for said variable across application, otherwise move error.
                 // Not thread safe.
struct Person {
    name: Rc<String>
}

impl Person { 
    fn new(name: Rc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}.", *self.name);    
    }   
}

fn main() {
    let name = Rc::new("John".to_string());

    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name)); // 1 for name.
    {
         let person = Person::new(name.clone()); // create unique ownership for each owner.
         println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name)); // 2 for name and person.
         person.greet();
    }
   
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name)); // 1 for name.
}
```

## Atomic Reference-Counted Variables (Arc)

```rust
use std::sync::Arc; // thread safe.
use std::thread;

struct Person {
    name: Arc<String>
}

impl Person { 
    fn new(name: Arc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}.", *self.name);    
    }   
}

fn main() {
    let name = Arc::new("John".to_string());
   
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();    
    });

    println!("Name = {}", name); 
 
    t.join().unwrap();
}
```

## Using a Mutex for Thread-Safe Mutability

```rust
use std::sync::{Mutex,Arc}; // thread safe.
use std::thread;

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl Person { 
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person { name: name, state: state }
    }

    fn greet(&self) {
        let mut state = self.state.lock().unwrap(); // locking it from other threads.
        self.state.clear();
        self.state.push_str("excited");
        

        println!("Hi, my name is {} and I am {}.", *self.name, state.as_str());   
    }   
}

fn main() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
   
    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet();    
    });

    println!("Name = {}, State = ", name, bored.lock().unwrap().as_str()); 
 
    t.join().unwrap();
}
```