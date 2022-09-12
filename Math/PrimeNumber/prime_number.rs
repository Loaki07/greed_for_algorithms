use std::io;

fn main() {
    // logic
    // n = underoot n * underroot n

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Enter a valid number!"),
    };

    if n == 2 {
        // edge case if input is 2
        println!("n is prime");
    } else {
        let mut is_prime: bool = true;

        // Code can be further optimized by checking until underoot n
        for i in 2..=((n as f64).sqrt() as usize) {
            if n % i == 0 {
                is_prime = false;
            }
        }

        if is_prime {
            println!("n is prime");
        } else {
            println!("n is not prime");
        }
    }
}
