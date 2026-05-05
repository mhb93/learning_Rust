fn main() {
    #[allow(non_snake_case)]   // I am a camelCase stan
    let mutableVariable = 14;
    println!("{mutableVariable}");
    #[allow(non_snake_case)] let mutableVariable = true;
    println!("{mutableVariable}");

    let _array: [u16; 6] = [394, 6512, 34800, 33, 10901, 64723];
    let _tuple = (18u32, 7u8, false, -1976f32);
}
