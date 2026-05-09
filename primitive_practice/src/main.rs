#[derive(Debug)]
struct Wrestler{
    name: String,
    finisher: String,
    age: u8,
}

#[allow(non_snake_case)] // I am a camelCase stan
struct Point{
    xCoord: f32,
    yCoord: f32,
}

#[allow(non_snake_case)] // I am a camelCase stan
struct Rectangle{
    topLeft: Point,
    topRight: Point,
}

#[allow(non_snake_case)] // I am a camelCase stan
enum AMPPSMembers{
    Quetzal,
    Mary,
    Jonathan,
    Bruce,
    Ayman,
}

const NUM_APPLES: i8 = 47;

#[allow(non_snake_case)] // I am a camelCase stan
fn main() {
    let mutableVariable = 14;
    println!("{mutableVariable}");
    let mutableVariable = true;
    println!("{mutableVariable}");

    // Declaring a couple different data structures
    let _array: [u16; 6] = [394, 6512, 34800, 33, 10901, 64723];
    let _tuple = (18u32, 7u8, false, -1976f32);

    // Shadowing example
    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

    // Look! You can assign an old variable to a new variable, even if its life is
        // fleetingly short
    let randomVar;
    {
        let theLetterA = 999;
        randomVar = theLetterA * theLetterA;
    }
    println!("{randomVar}");

    // Here is an exmpale of freezing:
    #[allow(unused_assignments)] let mut newVar = 945000i32;
    {
        // let newVar = newVar;    // Now he is frozen
        // If I were to uncomment the next two lines, we'd have issues
        // newVar = 9;
        // println!("{newVar}");
    }
    newVar = 392;
    println!("{newVar}\n");

    // Casting
    let _decimal = 53.444_f32;

    // Tuples n stuff
    let tuple1 = (91, 64, 'z', 281, 999, 475, "potatoes", 3.14);
    println!("{}", tuple1.2);
    println!("{}", tuple1.6);
    let metaTuple = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16, 'q');
    println!("Send him to the MEGA TUPLE ZONE {:?}", metaTuple);

    let thisThing = (NUM_APPLES, "glorp", 98.98, false);
    let (a, b, c, d) = thisThing;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    let arrayOne: [&str; 4] = ["Get", "off", "my", "lawn"];
    let arrayTwo: [i16; 392] = [-4; 392];
    for index in 0..arrayOne.len(){
        println!("{}, {}", arrayOne[index], index);
    }
    for index in 0..5{
        println!("{}", arrayTwo[index]);
    }

    // Size
    println!("The size of arrayTwo in bytes is: {}", std::mem::size_of_val(&arrayTwo));

    // Make some wrestlers
    let wrestName = "CM Punk".to_string();
    let wrestFinisher = "GTS".to_string();
    let wrestAge = 47;

    let Punk = Wrestler{ name: wrestName, finisher: wrestFinisher, age: wrestAge };
    let Danhausen = Wrestler{ name: "Danhausen".to_string(), finisher: "You Are Cursed!".to_string(), age: 35 };
    println!("{:?}", Punk);
    println!("My favorite wrestler is {}! His finisher is {}, and his age is {}.", Danhausen.name, Danhausen.finisher, Danhausen.age);

    // Practice with the "use" keyword
    let spaceCrew = Ayman;
    let spaceCrew2 = Quetzal;
    use AMPPSMembers::*;
    // I quite like this match stuff
    match spaceCrew{
        Quetzal => println!("I am the PI."),
        Mary => println!("I am the telemetry and high voltage guy."),
        Jonathan => println!("I am the 3D printing guy."),
        Bruce => println!("I am the memory and pulses guy."),
        Ayman => println!("I am the plastics guy."),
    }
    match spaceCrew2{
        Quetzal => println!("I am the PI."),
        Mary => println!("I am the telemetry and high voltage guy."),
        Jonathan => println!("I am the 3D printing guy."),
        Bruce => println!("I am the memory and pulses guy."),
        Ayman => println!("I am the plastics guy."),
    }

    // Parsing a string
    let parsed: i32 = "6".parse().unwrap();
    let megaParsed = "10".parse::<i32>().unwrap();
    let sum = parsed + megaParsed;
    println!("Sum: {}", sum);
}
