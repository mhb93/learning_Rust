// This program is based on the Saturday Night Live sketch Name That Dog!
// The idea is to guess the name of the dog by looking at it.
// It's still a work in progress...
// Last updated 05/08/2026

// The names of the dogs that the user will be shown throughout the game
const DOGNAMES: [&str; 18] = ["Cooper", "Lobo", "Shep", "Trooper", "Steve", "Lawrence", "Cynthia", "Gummo", "Boots", "Marshall", "Macy", "Peanut", "Dojo", "Max", "Douglas", "Shiloh", "Pixie", "Ruggles"];
const AARONSRESPONSE: &str = "I'M GONNA GUESS THAT DOG'S NAME!";


#[allow(non_snake_case)] // I am a camelCase stan
fn main() {
    // Introduction
    println!("Welcome to Name That Dog!");
    println!("(This game was based on the Saturday Night Live sketch of the same name. It's pretty funny and I think you should look it up.)");
    println!("Now, take a look at this dog: ");
    let aaronEasterEgg: String = AARONSRESPONSE.to_string();

    // Dogs
    let dogArray: [String; 3] = ["  __    __\no-''))_____\\\n\"--__/ * * * )\nc_c__/-c____/".to_string(),
                            "         __\n        /  \\\n       / ..|\\\n      (_\\  |_)\n      /  \\@' \n     /     \\\n_   /  `   |\n\\\\/  \\  | _\\\n \\   /_ || \\\\_\n  \\____)|_) \\_)".to_string(),
                            "             ____\n            /    \\__\n|\\         /    @   \\\n\\ \\_______|    \\   .:|>\n \\      ##|    | \\__/\n  |    ####\\__/   \\\n  /  /  ##       \\|\n /  /__________\\  \\\n L_JJ           \\__JJ".to_string(),
                            ];

    loop{
        let asciiNum: usize = rand::random_range(0..dogArray.len());
        let dogArt = &dogArray[asciiNum];
        println!("{dogArt}");
        println!("Name that dog!\n(Type in the dog's name and then press enter, or type \"quit\" to quit.)");
        // Choose the dog's name
        let dogNameNum: usize = rand::random_range(0..(DOGNAMES.len() - 1));
        let mut dogName = DOGNAMES[dogNameNum].to_string();

        // Take a guess from the user
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Oopsie, I didn't get that.");
        guess = guess.trim().to_string();


        // Important: user cannot get it right, ever
        // TODO: make this case-unsensitive
        while guess == aaronEasterEgg { // Had to throw an easter egg in here
            println!("Take it easy there, Aaron.");
            guess.clear();
            std::io::stdin().read_line(&mut guess).expect("Oopsie, I didn't get that.");
            guess = guess.trim().to_string();
        }
        if dogName == guess {
            dogName = (DOGNAMES[dogNameNum + 1]).to_string();
        }
        // Let them quit if they want
        else if guess == "quit" {
            println!("Thanks for playing!");
            break;
        }
        println!("Oh, I'm so sorry, but that's incorrect!");
        println!("This dog's name is {}.\n\n", dogName);
    }
}
