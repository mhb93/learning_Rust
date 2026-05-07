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

    let thisThing = (1, "glorp", 98.98, false);
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
}
