enum Colour {
    Red,
    Green,
    Blue,
    RgbColour(u8, u8, u8), // tuple-like construct
    CmykColour { cyan:u8, magenta:u8, yellow:u8, black:u8 }, // struct-like construct
}

pub fn enum_colours() {
    let c:Colour = Colour::Red;
    let d:Colour = Colour::RgbColour(0, 0, 0);
    let e:Colour = Colour::CmykColour{ cyan:0, magenta:128, yellow:0, black:0 };

    match_enum(c);
    match_enum(d);
    match_enum(e);
}

fn match_enum(colour: Colour) {
    match colour {
        Colour::Blue => println!("The colour is blue."),
        Colour::Red => println!("The colour is red."),
        Colour::Green => println!("The colour is green."),
        Colour::CmykColour { yellow:y, black:b, cyan:c, magenta:m} =>  println!("The CMYK is ({}, {}, {}, {}).", c, m, y, b),
        Colour::RgbColour(r, g, b) =>  println!("The RGB is ({}, {}, {}).", r, g, b),
    }
}

pub fn option_example() {
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
}