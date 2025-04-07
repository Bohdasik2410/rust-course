use std::io;

fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }
    for i in 2..=(*n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    println!("Введіть ціле число для перевірки:");

    io::stdin().read_line(&mut input).expect("Помилка зчитування");
    
    match input.trim().parse::<u32>() {
        Ok(number) => {
            if is_prime(&number) {
                println!("Число {} є простим", number);
            } else {
                println!("Число {} не є простим", number);
            }
        },
        Err(_) => println!("Будь ласка, введіть коректне ціле число"),
    }
}
