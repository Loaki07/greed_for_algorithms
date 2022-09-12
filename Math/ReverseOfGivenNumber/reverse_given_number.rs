fn main() {
    // logic
    // int n
    // rev = 0
    // lastDigit = n % 10
    // rev = (rev * 10) + lastDigit
    let mut n: u32 = 10899;
    let mut rev: u32 = 0;

    while n > 0 {
        let last_digit = n % 10;
        rev = (rev * 10) + last_digit;
        n = n / 10;
    }

    println!("Reverse of the number: {}", rev);
}
