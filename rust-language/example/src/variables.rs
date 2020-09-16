#[warn(dead_code)]

pub fn integers() {
    println!("Integers:");

    let a:i8 = -1; // immutable, 8 bit integer.
    let b:i16 = 1; // immutable, 16bit integer.
    let c = -1; // immutable and integer was inferred, defaults to 32bit integer.
    let d:i64 = 1; // immutable, 64bit integer.
    let e:i128 = -1; // immutable, 128bit integer.
    let os_integer:isize = -1; // uses the target to determine what size needs to be used. On 32bit OS it will 32bit integer and so on.

    println!("a = {}, b = {}, c = {}, d = {}, e = {}, os_integer = {}", a, b, c, d, e, os_integer);

    println!("Unsigned Integers:");

    let f:u8 = 1; // immutable, 8 bit unsigned integer.
    let g:u16 = 1; // immutable, 16 bit unsigned integer.
    let h:u32 = 1; // immutable, 32 bit unsigned integer.
    let i:u64 = 1; // immutable, 64 bit unsigned integer.
    let j:u128 = 1; // immutable, 128 bit unsigned integer.
    let os_unsigned_integer:usize = 1; // uses the target to determine what size needs to be used. On 32bit OS it will 32bit integer and so on.

    println!("f = {}, g = {}, h = {}, i = {}, j = {}, os_unsigned_integer = {}", f, g, h, i, j, os_unsigned_integer);
}
