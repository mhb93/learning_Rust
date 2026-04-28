fn main() {
    // Normal variables:
    let immutableImogen = 14;
    println!("The variable immutableImogen, whose value is {immutableImogen}, is immutable.");
    let mut mutableMandy = 999;
    println!("The variable mutableMandy, whose value is {mutableMandy}, is mutable.");
    println!("Now I will change mutableMandy.");
    mutableMandy = 348;
    println!("mutableMandy's value is {mutableMandy}.\n");

    // Constant:
    const constantCaroline: u16 = 456;
    println!("constantCaroline is a const. Its value is {constantCaroline}.\n");

    // Practice with shadow variables:
    let stinkySteve = 102;
    {
        let stinkySteve = stinkySteve * 2;
        println!("stinkySteve has a value of {stinkySteve}.");
    }
    println!("Just kidding! stinkySteve has a value of {stinkySteve}.\n");
}