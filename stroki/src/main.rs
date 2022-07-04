fn main() {
    let s1 = String::new(); // Создание строки
    let s2 = String::from("Hello"); // Строка с значением (1 способ)
    let mut s3 = "Rust".to_string(); // Строка со значеним (2 способ)
    s3.push_str(" World!"); // Добавление к string
    println!("{}", s3);
}