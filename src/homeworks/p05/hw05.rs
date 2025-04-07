use std::io;

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Введіть перше число:");
    io::stdin().read_line(&mut input1).expect("Помилка при зчитуванні числа");
    let a: u32 = input1.trim().parse().expect("Введіть коректне ціле число");

    println!("Введіть друге число:");
    io::stdin().read_line(&mut input2).expect("Помилка при зчитуванні числа");
    let b: u32 = input2.trim().parse().expect("Введіть коректне ціле число");

    println!("НСД чисел {} і {} дорівнює {}", a, b, gcd(a, b));
}
