use std::io;

fn invert_the_case(s: String) -> String {
    s.chars()
     .map(|c| {
         if c.is_lowercase() {
             c.to_uppercase().to_string()
         } else if c.is_uppercase() {
             c.to_lowercase().to_string()
         } else {
             c.to_string()
         }
     })
     .collect()
}

fn main() {
    let mut input = String::new();

    println!("Введіть текст для зміни регістру:");
    io::stdin().read_line(&mut input).expect("Не вдалося зчитати рядок");

    let result = invert_the_case(input.trim().to_string());
    println!("Результат: {}", result);
}
