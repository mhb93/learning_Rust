// I am going to practice using macros

macro_rules! smoke_a_cigarette {
    () => { // No parameters
        // Smoke a cigarette
        println!("You have smoked a cigarette. You are 1 day closer to lung cancer.")
    };
}

fn main() {
    smoke_a_cigarette!()
}
