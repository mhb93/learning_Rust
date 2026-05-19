// This program is designed to find the Nth Fibonacci number, where N is a number given by the user
// Last updated 5/18/2026 

const FIBO_FINAL: usize = 40;

#[allow(non_snake_case)]
fn buildFiboArray(array: &mut [usize; FIBO_FINAL], last_num: usize, current_num: i16) {
    // Build the Fibonnaci array
    // Edge cases
    assert!((current_num as usize) <= last_num);
    // The end
    if current_num == last_num as i16 {
        // We have reached the end
        return;
    }
    // The beginning
    else if current_num == 0 {
        array[0] = 0;
    }
    // The other beginning
    else if current_num == 1 {
        array[1] = 1;
    }

    // Otherwise, Fibo your heart out
    else{
        array[current_num as usize] = array[(current_num - 1) as usize] + array[(current_num - 2) as usize];
    }
    buildFiboArray(array, last_num, current_num + 1);
    return;
}

fn main() {
    let mut array: [usize; FIBO_FINAL] = [0; FIBO_FINAL];
    let current_index: i16 = 0;
    buildFiboArray(&mut array, FIBO_FINAL, current_index);
    let mut fibo_index: String = String::new();
    loop{
        println!("Give me a number and I'll give you that number in the Fibonacci sequence.");
        std::io::stdin().read_line(&mut fibo_index).expect("Uh oh~");
        fibo_index = fibo_index.trim().to_string();

        // Convert that string to an integer
        let fibo_index_num: usize = fibo_index.parse::<>().unwrap();

        println!("The {} Fibonacci number in the sequence is {}", fibo_index_num, array[fibo_index_num]);
        fibo_index.clear();
        // fibo_index_num.clear();
    }
}