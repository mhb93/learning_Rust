#[allow(non_snake_case)] // I am a camelCase stan


fn main() {
    let mutableVariable = 14;
    println!("{mutableVariable}");
    #[allow(non_snake_case)] let mutableVariable = true;
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
    let mut newVar = 945000i32;
    {
        let newVar = newVar;    // Now he is frozen
        // If I were to uncomment the next two lines, we'd have issues
        // newVar = 9;
        // println!("{newVar}");
    }
    newVar = 392;
    println!("{newVar}");

    // Casting
    let decimal = 53.444_f32;
}
