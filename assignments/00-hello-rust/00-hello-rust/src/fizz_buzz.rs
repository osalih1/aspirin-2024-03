pub fn print_fizz_buzz(max_num: u32) {
    for num in 1..=max_num {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", num);
        }
    }
}

fn main() {
    print_fizz_buzz(15);
}
