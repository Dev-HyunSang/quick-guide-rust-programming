// 2024.11.28 함수와 루프

fn fizzbuzz_if_else(max: i32) {
    for i in 1..=max {
        let rem_three: i32 = i % 3;
        let rem_five: i32 = i % 5;

        if rem_three == 0 && rem_five == 0 {
            println!("{} - FizzBuzz", i);
        } else if rem_three == 0 {
            println!("{} - Fizz", i);
        } else if rem_five == 0 {
            println!("{} - Buzz", i);
        } else {
            /* do noting */
        }
    }
}

fn main() {
    println!("Hello, function_for!");
    fizzbuzz_if_else(10);
}
